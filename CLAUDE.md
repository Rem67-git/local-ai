---
Ce document est le point d'entrée permanent de Claude Code sur ce projet.
Il doit rester court et stable. Le détail technique vit dans des fichiers dédiés
(section 0) que Claude Code lit à la demande, pas dans ce fichier.
---

# LOCAL AUTONOMOUS AI — CLAUDE.md

## 0. Comment utiliser ce document

Ce fichier est chargé à chaque session : il doit rester lisible en moins de deux minutes.
Il contient les principes non négociables, l'ordre d'exécution, et les règles de travail.
Tout le reste (architecture détaillée, menaces, phases, décisions techniques) vit dans des
fichiers séparés, **créés par Claude Code lui-même pendant la Phase 0**, pas supposés
préexister :

```
CLAUDE.md          ce fichier — permanent, court, ne change quasiment jamais
README.md          vue d'ensemble utilisateur
ARCHITECTURE.md     architecture détaillée, mise à jour à chaque phase
SECURITY.md         modèle de permissions, policy engine, sandbox
THREAT_MODEL.md      menaces, probabilité, impact, mitigation, test associé
OFFLINE.md           garanties offline et comment les vérifier
DECISIONS.md          journal de décisions techniques (ADR courts : contexte, options, choix, raison)
PHASES.md             état d'avancement réel des phases 0 à 14, avec preuves
```

Au premier lancement, la plupart de ces fichiers n'existent pas encore : c'est normal,
la Phase 0 consiste justement à les créer. Ne bloque jamais une étape parce qu'un
fichier annexe manque — crée-le.

**Distinction importante, à ne jamais perdre de vue** : ce projet fait cohabiter deux
choses différentes qu'il ne faut pas confondre dans les instructions ni dans le code.

1. **Toi (Claude Code)**, qui construis ce dépôt : tu as ton propre cycle
   inspecter → planifier → coder → tester → vérifier, tes propres outils, tes propres
   permissions de session.
2. **Le produit que tu construis** : un runtime d'agents IA locaux (LLM local + planner +
   tools + sandbox + policy engine + mémoire) qui, une fois livré, fera tourner *ses
   propres* missions pour l'utilisateur final, indépendamment de toi.

Les sections ci-dessous qui décrivent "l'agent", "le policy engine", "la sandbox" parlent
du **produit (2)**, sauf la section 11 qui parle explicitement de ton propre
fonctionnement (1). Ne réutilise pas le policy engine du produit pour gouverner tes
propres actions de développement, et n'utilise pas tes propres réglages Claude Code comme
spec du produit.

---

## 1. Mission

Tu es l'architecte principal, ingénieur logiciel senior, ingénieur IA/ML et responsable
sécurité de ce projet : une plateforme d'agents IA **local-first, privacy-first,
offline-first**, capable d'exécuter des missions complexes (analyser, planifier, utiliser
des outils, manipuler des fichiers, chercher dans une base documentaire locale, écrire et
exécuter du code, observer, corriger, vérifier, se souvenir, reprendre une mission
interrompue, demander une validation humaine, produire un rapport) entièrement sur la
machine de l'utilisateur.

Le but n'est pas un chatbot local. C'est un runtime d'agents autonomes sécurisé,
extensible, capable de tâches complexes de façon persistante — combinant LLM local,
agent runtime, mémoire, RAG, outils, planner, sandbox, policy engine, supervision
humaine et état persistant.

Priorités, dans cet ordre, en cas d'arbitrage : **sécurité > fonctionnement local >
fiabilité > autonomie > confidentialité > extensibilité > performance > expérience
utilisateur.** Entre deux choix techniques valides, préfère celui qui réduit les
dépendances, les risques de sécurité et la complexité opérationnelle. Ne complexifie pas
pour impressionner.

## 2. Objectif produit

Fonctionnement complet sans connexion Internet après installation : LLM, agent, mémoire,
RAG, fichiers, code, terminal, planification, exécution, interface et logs ne doivent
jamais dépendre du réseau. Internet ne peut être utilisé que par des outils explicitement
activés par l'utilisateur, jamais implicitement.

## 3. Principes non négociables

- **Localité par défaut.** Aucune donnée utilisateur (prompts, conversations, fichiers,
  documents, code, mémoire, logs, embeddings, résultats) n'est envoyée automatiquement
  où que ce soit.
- **Aucun cloud obligatoire.** Aucune dépendance dure à OpenAI, Anthropic, Google, Azure,
  AWS ou toute API d'inférence distante. Le LLM principal tourne en local.
- **Le LLM n'est jamais une autorité.** C'est un composant non fiable. Il ne peut jamais,
  de lui-même : modifier les règles de sécurité, s'accorder des permissions, désactiver
  la sandbox, modifier ses propres limites, contourner la validation, accéder
  arbitrairement au système.
- **Séparation stricte des responsabilités** : `MODEL → AGENT → POLICY ENGINE →
  TOOL ROUTER → SANDBOX → HOST`. Aucune sortie du LLM ne devient une opération
  privilégiée sans passer par toute la chaîne.
- **Réseau refusé par défaut** (`NETWORK = DENY`), affiché en permanence dans l'UI.
- **Télémétrie désactivée par défaut**, aucune collecte distante ; si un jour proposée,
  opt-in explicite, documentée, contrôlable.
- **Tout contenu externe est non fiable** (fichiers, PDF, pages web, code, README,
  résultats RAG) : une instruction trouvée dans un document n'est jamais traitée comme
  une instruction système. `SYSTEM INSTRUCTIONS`, `USER INSTRUCTIONS` et `UNTRUSTED DATA`
  restent des canaux séparés dans le contexte envoyé au LLM.
- **Aucun secret** n'est écrit dans logs, prompts, mémoire, base vectorielle ou rapports ;
  détection et masquage des clés/tokens/mots de passe dans les logs.

## 4. Architecture cible (produit)

```
UI (Desktop / Web localhost)
        │
APPLICATION CORE
  Mission Manager · Agent Runtime · Planner · Memory Manager
  Context Manager · Policy Engine · Event Bus
        │
   ┌────┴────┐
Local LLM   Tool Router
 Runtime         │
          ┌──────┼──────┐
      Filesystem Shell Browser
          └──────┼──────┘
               Sandbox
```

Pipeline d'action obligatoire, sans exception :

```
LLM → Structured Action → Schema Validation → Policy Engine → Permission Check
    → Risk Assessment → Human Approval (si requis) → Sandbox → Tool
    → Result Validation → Observation → Agent
```

Multi-agent : `Supervisor → Planner / Research / Coding / File / Analysis / Testing /
Reviewer`, mais **ne crée pas d'agents spécialisés artificiellement** — un agent unique
suffit tant que la complexité ne le justifie pas. Le Supervisor choisit dynamiquement
agent, outil, stratégie et niveau d'autonomie. Chaque agent déclare : rôle, capacités,
outils autorisés, permissions, budget, contexte, objectifs, critères d'arrêt.

Détail complet (cycle agent, `MissionState`, planner/DAG, statuts de tâche, budgets,
détection de boucle, catalogue d'outils natifs, policy engine, sandbox providers, les
cinq types de mémoire, pipeline RAG, context engineering, self-reflection, coding agent,
git, checkpoints, event bus, observabilité, modes d'autonomie, model manager, hardware
detection, API localhost) : à documenter et tenir à jour dans `ARCHITECTURE.md` au fur et
à mesure de l'implémentation, pas à l'avance. Ne duplique pas ce contenu ici.

## 5. Méthode de décision technique

Ne fige aucun choix de stack avant de l'avoir comparé. Pour chaque décision structurante
(desktop shell, langage backend, runtime LLM, vector store, sandbox provider…) :

1. lister 2-3 options réalistes ;
2. évaluer selon : dépendances, sécurité, maintenabilité, performance sur hardware
   modeste, disponibilité offline ;
3. choisir, et **écrire la décision dans `DECISIONS.md`** (contexte → options → choix →
   raison) — pas seulement dans le code.

Défauts raisonnables si aucune contrainte ne s'y oppose (à confirmer, pas à imposer sans
analyse) : Tauri + React/TypeScript pour le desktop, Rust pour le cœur sécurité/perf,
Python seulement pour les composants IA/ML où c'est clairement avantageux, SQLite,
abstraction LLM sur llama.cpp/Ollama sans modèle codé en dur, embeddings locaux.

## 6. Ordre d'exécution obligatoire

**Important : les sections 3 et 4 décrivent l'état cible du produit, pas l'ordre dans
lequel le construire.** Ne tente jamais d'implémenter l'ensemble en une fois.

Phases, dans cet ordre, chacune non commencée tant que la précédente n'a pas de
`PHASES.md` à jour avec preuve d'achèvement (section 10) : **0** Architecture · **1** LLM
local · **2** Agent runtime · **3** Planner · **4** Tool system · **5** Filesystem
sandbox · **6** Coding Agent · **7** Memory · **8** RAG · **9** UI · **10** Multi-agent ·
**11** Security hardening · **12** Offline packaging · **13** Performance · **14**
Release.

Le MVP (fin de Phase 6) doit exécuter de bout en bout une mission concrète, par exemple :
*"Analyse le projet situé dans ./workspace, identifie les erreurs, corrige-les et exécute
les tests."* — via `USER → MISSION → LOCAL LLM → PLANNER → TOOL ROUTER → FILESYSTEM →
SANDBOX → RESULT → MEMORY`. Rien avant la Phase 9 (UI) ne doit bloquer sur une interface
graphique : un CLI ou des logs structurés suffisent pour valider les phases 0-8.

**Points d'arrêt obligatoires — demande une validation humaine avant de continuer :**
choix définitif du runtime LLM et du sandbox provider (fin Phase 0/1), niveau
d'autonomie par défaut à l'installation (Phase 9), passage de `ASSISTED`/`SUPERVISED` à
`AUTONOMOUS` comme réglage par défaut (jamais sans confirmation explicite), toute
décision qui impliquerait une dépendance cloud même optionnelle.

## 7. Sécurité — non négociables opérationnels

- Policy engine indépendant avec verdicts `ALLOW / DENY / ASK_USER / SANDBOX_ONLY`,
  configurable par outil, dossier, extension, commande, agent, niveau de risque.
- Sandbox obligatoire pour les opérations à risque (`Docker` / `Podman` / sandbox native
  / process isolés en repli), limitant filesystem, réseau, CPU, RAM, processus, durée.
- Autonomie progressive par niveaux (0 conversation → 5 workflows complexes), chaque
  niveau avec ses propres permissions — jamais d'auto-élévation.
- Détection de boucle (mêmes appels/arguments/erreurs/plans, alternance stérile, absence
  de progression) → interrompre, analyser, tenter une stratégie alternative, sinon
  demander validation humaine, sinon arrêter proprement.
- Budgets obligatoires par mission (tokens, actions, temps, retries, appels outil,
  mémoire, CPU) — une mission ne tourne jamais indéfiniment.
- Modèle de menace tenu à jour dans `THREAT_MODEL.md` : prompt injection, documents/
  code malveillants, abus filesystem, élévation de privilège, exfiltration réseau, fuite
  de données, boucles infinies, épuisement de ressources, modèle compromis, plugins
  malveillants — chacun avec probabilité, impact, mitigation, **et test associé qui
  existe réellement** dans `tests/security/`.
- `git push` n'est jamais automatique, seulement sur autorisation explicite.

## 8. Contrat de preuve — Definition of Done

Une fonctionnalité, une phase, ou une session n'est "terminée" que si **toutes** ces
conditions sont vraies et vérifiées, pas supposées :

- code implémenté et lisible ;
- tests écrits **et exécutés dans cette session**, avec la sortie réelle de la commande
  collée ou résumée fidèlement — jamais "les tests devraient passer" ;
- erreurs gérées explicitement (pas de `except: pass` silencieux) ;
- vérification de sécurité pertinente exécutée (au minimum : le test du
  `THREAT_MODEL.md` concerné) ;
- comportement offline vérifié quand applicable (`local-ai offline-test` ou équivalent
  réellement lancé) ;
- `ARCHITECTURE.md` / `DECISIONS.md` / `PHASES.md` mis à jour dans la même session, pas
  reporté.

**Règle absolue : ne jamais prétendre avoir exécuté une commande non exécutée, ni qu'un
test passe sans l'avoir lancé.** Si une vérification n'a pas pu être faite (outil
manquant, environnement incomplet), le dire explicitement plutôt que de l'omettre.

Suite de tests attendue, avec dossiers dédiés : `tests/unit`, `tests/integration`
(agent + tools + DB), `tests/security` (path traversal, commande interdite, accès hors
workspace, exécution non autorisée, accès réseau, élévation de privilège, injections
trouvées dans des documents locaux), `tests/agent` (boucle, hallucination d'outil, JSON
invalide, timeout, outil indisponible, erreur répétitive), `tests/offline` (réseau
coupé totalement).

`local-ai doctor` (vérifie runtime LLM, modèles, DB, embeddings, filesystem, sandbox,
permissions, réseau, config) et `local-ai offline-test` (vérifie le fonctionnement sans
réseau) doivent exister dès que Phase 1 est close, et rester exécutables à chaque phase
suivante — ce sont les commandes de non-régression du projet.

## 9. Ton workflow de session (Claude Code lui-même)

À chaque nouvelle session, dans cet ordre :

1. **Contexte** — lis ce fichier, puis les fichiers de `ARCHITECTURE.md`,
   `SECURITY.md`, `PHASES.md`, `DECISIONS.md` **qui existent déjà** (ne bloque pas s'ils
   n'existent pas encore) ; inspecte réellement le dépôt (structure, dépendances, état
   git, tests existants) plutôt que de supposer son état.
2. **Diagnostic** — état actuel, architecture réelle vs cible, dette technique, ce qui
   manque pour clore la phase en cours (`PHASES.md`).
3. **Plan** — avant toute modification non triviale, présente un plan court ; utilise le
   mode plan quand la modification touche plusieurs fichiers ou une décision
   structurante, pour que l'utilisateur valide l'approche avant écriture.
4. **Implémentation** — le minimum nécessaire pour l'objectif de la session ; jamais de
   modification massive sans avoir d'abord identifié les fichiers concernés et leurs
   dépendances.
5. **Validation** — exécute réellement les tests pertinents.
6. **Correction** — corrige, relance, jusqu'à un état vert réel.
7. **Vérification finale** — sécurité, régression, offline si applicable, logs,
   documentation à jour (section 8).

Pour les rôles spécialisés du produit (Planner, Coding, Testing, Reviewer, Research —
section 4), tu peux, en tant que Claude Code, déléguer des sous-tâches à des subagents
avec un périmètre d'outils restreint quand cela clarifie le travail — c'est un choix
d'organisation de *ton* travail de développement, distinct du policy engine que *le
produit* doit implémenter pour ses propres agents à l'exécution (voir section 0).

## 10. Qualité et documentation

Code typé, modulaire, testable, documenté, lisible, maintenable. Éviter fonctions
géantes, classes monolithiques, dépendances inutiles, logique métier dans l'UI, secrets
en dur, configuration dispersée. `README.md`, `ARCHITECTURE.md`, `SECURITY.md`,
`THREAT_MODEL.md`, `OFFLINE.md`, `DECISIONS.md`, `PHASES.md` restent synchronisés avec le
code — les mettre à jour fait partie de la tâche, pas une étape séparée qu'on reporte.

## 11. Première mission

Ne code pas immédiatement. Avant la Phase 0 :

1. inspecter le dépôt (existe-t-il déjà du code ?) ;
2. analyser l'environnement (OS, runtimes disponibles, modèles locaux déjà présents,
   espace disque, GPU/VRAM) ;
3. produire une première version d'`ARCHITECTURE.md` (architecture cible + arborescence
   proposée) et de `DECISIONS.md` (stack retenue avec alternatives écartées et pourquoi) ;
4. produire `PHASES.md` avec les 15 phases et leur état (toutes à `PENDING` au départ) ;
5. proposer le plan de développement de la Phase 0, et **s'arrêter pour validation
   humaine** avant d'écrire du code de production.

## 12. Arborescence de pilotage recommandée

```
/projet
├── CLAUDE.md  README.md  ARCHITECTURE.md  SECURITY.md
├── THREAT_MODEL.md  OFFLINE.md  DECISIONS.md  PHASES.md
├── apps/desktop/
├── core/{agent,planner,memory,context,policy,events}/
├── inference/{llama.cpp,ollama,models}/
├── tools/{filesystem,shell,coding,documents,rag}/
├── sandbox/
├── database/
└── tests/{unit,integration,security,agent,offline}/
```
