import { useState } from 'react'
import { Card } from '../components/ui/Card'
import { 
  Settings as SettingsIcon, 
  Bell, 
  Shield, 
  Zap, 
  DollarSign,
  Moon,
  Sun,
  Smartphone,
  Mail,
  AlertCircle,
  Save
} from 'lucide-react'

interface NotificationSettings {
  tradeAlerts: boolean
  priceAlerts: boolean
  systemUpdates: boolean
  marketingEmails: boolean
  smsNotifications: boolean
}

interface TradingSettings {
  autoAcceptThreshold: number
  maxDailyTrades: number
  preferredEnergyTypes: string[]
  priceAlertThreshold: number
}

interface SecuritySettings {
  twoFactorAuth: boolean
  biometricAuth: boolean
  sessionTimeout: number
  emailVerification: boolean
}

export default function Settings() {
  const [activeTab, setActiveTab] = useState<'general' | 'notifications' | 'trading' | 'security'>('general')
  const [darkMode, setDarkMode] = useState(false)
  const [language, setLanguage] = useState('en')
  const [currency, setCurrency] = useState('USD')
  
  const [notifications, setNotifications] = useState<NotificationSettings>({
    tradeAlerts: true,
    priceAlerts: true,
    systemUpdates: true,
    marketingEmails: false,
    smsNotifications: true
  })

  const [trading, setTrading] = useState<TradingSettings>({
    autoAcceptThreshold: 0.10,
    maxDailyTrades: 10,
    preferredEnergyTypes: ['Solar', 'Wind'],
    priceAlertThreshold: 0.05
  })

  const [security, setSecurity] = useState<SecuritySettings>({
    twoFactorAuth: true,
    biometricAuth: false,
    sessionTimeout: 30,
    emailVerification: true
  })

  const tabs = [
    { id: 'general', name: 'General', icon: SettingsIcon },
    { id: 'notifications', name: 'Notifications', icon: Bell },
    { id: 'trading', name: 'Trading', icon: Zap },
    { id: 'security', name: 'Security', icon: Shield }
  ] as const

  const energyTypes = ['Solar', 'Wind', 'Hydro', 'Battery', 'Nuclear', 'Gas']

  const handleSave = () => {
    // Save settings logic here
    console.log('Settings saved!')
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
        <button
          onClick={handleSave}
          className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
        >
          <Save className="h-4 w-4 mr-2" />
          Save Changes
        </button>
      </div>

      <div className="flex flex-col lg:flex-row gap-6">
        {/* Sidebar */}
        <div className="lg:w-64">
          <Card className="p-4">
            <nav className="space-y-2">
              {tabs.map((tab) => {
                const Icon = tab.icon
                return (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                      activeTab === tab.id
                        ? 'bg-primary-50 text-primary-600 border-primary-200'
                        : 'text-gray-700 hover:bg-gray-50'
                    }`}
                  >
                    <Icon className="h-4 w-4 mr-3" />
                    {tab.name}
                  </button>
                )
              })}
            </nav>
          </Card>
        </div>

        {/* Content */}
        <div className="flex-1">
          {activeTab === 'general' && (
            <Card className="p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-6">General Settings</h3>
              
              <div className="space-y-6">
                {/* Appearance */}
                <div>
                  <h4 className="text-sm font-medium text-gray-900 mb-3">Appearance</h4>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      {darkMode ? <Moon className="h-4 w-4 mr-2" /> : <Sun className="h-4 w-4 mr-2" />}
                      <span className="text-sm text-gray-700">Dark Mode</span>
                    </div>
                    <button
                      onClick={() => setDarkMode(!darkMode)}
                      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                        darkMode ? 'bg-primary-600' : 'bg-gray-200'
                      }`}
                    >
                      <span
                        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                          darkMode ? 'translate-x-6' : 'translate-x-1'
                        }`}
                      />
                    </button>
                  </div>
                </div>

                {/* Language */}
                <div>
                  <h4 className="text-sm font-medium text-gray-900 mb-3">Language & Region</h4>
                  <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <div>
                      <label className="block text-sm text-gray-700 mb-2">Language</label>
                      <select
                        value={language}
                        onChange={(e) => setLanguage(e.target.value)}
                        className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      >
                        <option value="en">English</option>
                        <option value="es">Español</option>
                        <option value="fr">Français</option>
                        <option value="de">Deutsch</option>
                      </select>
                    </div>
                    <div>
                      <label className="block text-sm text-gray-700 mb-2">Currency</label>
                      <select
                        value={currency}
                        onChange={(e) => setCurrency(e.target.value)}
                        className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      >
                        <option value="USD">USD ($)</option>
                        <option value="EUR">EUR (€)</option>
                        <option value="GBP">GBP (£)</option>
                        <option value="JPY">JPY (¥)</option>
                      </select>
                    </div>
                  </div>
                </div>
              </div>
            </Card>
          )}

          {activeTab === 'notifications' && (
            <Card className="p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-6">Notification Preferences</h3>
              
              <div className="space-y-6">
                {Object.entries(notifications).map(([key, value]) => {
                  const labels = {
                    tradeAlerts: 'Trade Alerts',
                    priceAlerts: 'Price Alerts',
                    systemUpdates: 'System Updates',
                    marketingEmails: 'Marketing Emails',
                    smsNotifications: 'SMS Notifications'
                  }
                  
                  const descriptions = {
                    tradeAlerts: 'Get notified when trades are completed or require action',
                    priceAlerts: 'Receive alerts when energy prices change significantly',
                    systemUpdates: 'Important platform updates and maintenance notifications',
                    marketingEmails: 'Promotional emails and newsletter',
                    smsNotifications: 'Receive critical alerts via SMS'
                  }

                  const icons = {
                    tradeAlerts: Zap,
                    priceAlerts: DollarSign,
                    systemUpdates: AlertCircle,
                    marketingEmails: Mail,
                    smsNotifications: Smartphone
                  }

                  const Icon = icons[key as keyof typeof icons]

                  return (
                    <div key={key} className="flex items-center justify-between py-3 border-b border-gray-200 last:border-b-0">
                      <div className="flex items-start">
                        <Icon className="h-5 w-5 text-gray-400 mr-3 mt-0.5" />
                        <div>
                          <div className="text-sm font-medium text-gray-900">
                            {labels[key as keyof typeof labels]}
                          </div>
                          <div className="text-sm text-gray-500">
                            {descriptions[key as keyof typeof descriptions]}
                          </div>
                        </div>
                      </div>
                      <button
                        onClick={() => setNotifications(prev => ({ ...prev, [key]: !value }))}
                        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                          value ? 'bg-primary-600' : 'bg-gray-200'
                        }`}
                      >
                        <span
                          className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                            value ? 'translate-x-6' : 'translate-x-1'
                          }`}
                        />
                      </button>
                    </div>
                  )
                })}
              </div>
            </Card>
          )}

          {activeTab === 'trading' && (
            <Card className="p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-6">Trading Preferences</h3>
              
              <div className="space-y-6">
                <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Auto-Accept Threshold
                    </label>
                    <div className="relative">
                      <input
                        type="number"
                        step="0.01"
                        value={trading.autoAcceptThreshold}
                        onChange={(e) => setTrading(prev => ({ ...prev, autoAcceptThreshold: parseFloat(e.target.value) }))}
                        className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                      />
                      <span className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-gray-500">
                        $/kWh
                      </span>
                    </div>
                    <p className="mt-1 text-sm text-gray-500">
                      Automatically accept trades at or above this price
                    </p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Max Daily Trades
                    </label>
                    <input
                      type="number"
                      value={trading.maxDailyTrades}
                      onChange={(e) => setTrading(prev => ({ ...prev, maxDailyTrades: parseInt(e.target.value) }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                    />
                    <p className="mt-1 text-sm text-gray-500">
                      Maximum number of trades per day
                    </p>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-3">
                    Preferred Energy Types
                  </label>
                  <div className="flex flex-wrap gap-2">
                    {energyTypes.map((type) => (
                      <button
                        key={type}
                        onClick={() => {
                          setTrading(prev => ({
                            ...prev,
                            preferredEnergyTypes: prev.preferredEnergyTypes.includes(type)
                              ? prev.preferredEnergyTypes.filter(t => t !== type)
                              : [...prev.preferredEnergyTypes, type]
                          }))
                        }}
                        className={`px-3 py-1 rounded-full text-sm font-medium transition-colors ${
                          trading.preferredEnergyTypes.includes(type)
                            ? 'bg-primary-100 text-primary-700 border border-primary-200'
                            : 'bg-gray-100 text-gray-700 border border-gray-200 hover:bg-gray-200'
                        }`}
                      >
                        {type}
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Price Alert Threshold
                  </label>
                  <div className="relative">
                    <input
                      type="number"
                      step="0.01"
                      value={trading.priceAlertThreshold}
                      onChange={(e) => setTrading(prev => ({ ...prev, priceAlertThreshold: parseFloat(e.target.value) }))}
                      className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                    />
                    <span className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-gray-500">
                      $/kWh
                    </span>
                  </div>
                  <p className="mt-1 text-sm text-gray-500">
                    Get notified when prices change by this amount
                  </p>
                </div>
              </div>
            </Card>
          )}

          {activeTab === 'security' && (
            <Card className="p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-6">Security Settings</h3>
              
              <div className="space-y-6">
                {Object.entries(security).map(([key, value]) => {
                  if (key === 'sessionTimeout') {
                    return (
                      <div key={key}>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                          Session Timeout
                        </label>
                        <select
                          value={value}
                          onChange={(e) => setSecurity(prev => ({ ...prev, sessionTimeout: parseInt(e.target.value) }))}
                          className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                        >
                          <option value={15}>15 minutes</option>
                          <option value={30}>30 minutes</option>
                          <option value={60}>1 hour</option>
                          <option value={120}>2 hours</option>
                          <option value={480}>8 hours</option>
                        </select>
                        <p className="mt-1 text-sm text-gray-500">
                          Automatically log out after this period of inactivity
                        </p>
                      </div>
                    )
                  }

                  const labels = {
                    twoFactorAuth: 'Two-Factor Authentication',
                    biometricAuth: 'Biometric Authentication',
                    emailVerification: 'Email Verification'
                  }
                  
                  const descriptions = {
                    twoFactorAuth: 'Add an extra layer of security to your account',
                    biometricAuth: 'Use fingerprint or face recognition to log in',
                    emailVerification: 'Verify your email for account recovery'
                  }

                  return (
                    <div key={key} className="flex items-center justify-between py-3 border-b border-gray-200 last:border-b-0">
                      <div>
                        <div className="text-sm font-medium text-gray-900">
                          {labels[key as keyof typeof labels]}
                        </div>
                        <div className="text-sm text-gray-500">
                          {descriptions[key as keyof typeof descriptions]}
                        </div>
                      </div>
                      <button
                        onClick={() => setSecurity(prev => ({ ...prev, [key]: !value }))}
                        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                          value ? 'bg-primary-600' : 'bg-gray-200'
                        }`}
                      >
                        <span
                          className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                            value ? 'translate-x-6' : 'translate-x-1'
                          }`}
                        />
                      </button>
                    </div>
                  )
                })}
              </div>
            </Card>
          )}
        </div>
      </div>
    </div>
  )
}
