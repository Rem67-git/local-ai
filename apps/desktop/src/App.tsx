import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

function App() {
  const [goal, setGoal] = useState('')
  const [missions, setMissions] = useState<string[]>([])
  const [status, setStatus] = useState('idle')
  const [logs, setLogs] = useState<string[]>([])

  async function runMission() {
    if (!goal.trim()) return

    setStatus('running')
    addLog(`Starting mission: ${goal}`)

    try {
      const result = await invoke('run_mission', { goal })
      addLog(`Mission result: ${result}`)
      setStatus('idle')
      setGoal('')
    } catch (error) {
      addLog(`Error: ${error}`)
      setStatus('error')
    }
  }

  async function listMissions() {
    try {
      const result = await invoke('list_missions') as string[]
      setMissions(result)
      addLog(`Found ${result.length} missions`)
    } catch (error) {
      addLog(`Error listing missions: ${error}`)
    }
  }

  async function runDoctor() {
    try {
      const result = await invoke('doctor')
      addLog(`Doctor check: ${JSON.stringify(result)}`)
    } catch (error) {
      addLog(`Doctor failed: ${error}`)
    }
  }

  function addLog(message: string) {
    const timestamp = new Date().toLocaleTimeString()
    setLogs(prev => [...prev, `[${timestamp}] ${message}`])
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🤖 Local AI Agent</h1>
        <span className={`status ${status}`}>{status.toUpperCase()}</span>
      </header>

      <main className="app-main">
        <div className="mission-panel">
          <h2>Mission</h2>
          <div className="input-group">
            <textarea
              value={goal}
              onChange={(e) => setGoal(e.target.value)}
              placeholder="Enter mission goal..."
              rows={4}
            />
          </div>
          <div className="button-group">
            <button onClick={runMission} disabled={status !== 'idle'}>
              Run Mission
            </button>
            <button onClick={listMissions}>List Missions</button>
            <button onClick={runDoctor}>Doctor Check</button>
          </div>
        </div>

        <div className="missions-panel">
          <h2>Recent Missions</h2>
          <ul>
            {missions.map((m) => (
              <li key={m}>{m}</li>
            ))}
          </ul>
        </div>

        <div className="logs-panel">
          <h2>Logs</h2>
          <div className="logs-container">
            {logs.map((log, i) => (
              <div key={i} className="log-entry">{log}</div>
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}

export default App
