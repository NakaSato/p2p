import { Routes, Route } from 'react-router-dom'
import { Toaster } from 'sonner'
import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import Trading from './pages/Trading'
import Profile from './pages/Profile'
import Settings from './pages/Settings'
import CampusDemo from './pages/CampusDemo'
import CampusMapPage from './pages/CampusMapPage'
import './App.css'

function App() {
  return (
    <>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/trading" element={<Trading />} />
          <Route path="/profile" element={<Profile />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/campus-map" element={<CampusMapPage />} />
          <Route path="/campus-demo" element={<CampusDemo />} />
        </Routes>
      </Layout>
      <Toaster />
    </>
  )
}

export default App
