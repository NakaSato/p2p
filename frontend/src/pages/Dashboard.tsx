import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { 
  Zap, 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Users,
  Activity,
  Battery,
  Sun
} from 'lucide-react'

const stats = [
  {
    name: 'Energy Sold',
    value: '2,450 kWh',
    change: '+12%',
    changeType: 'increase' as const,
    icon: TrendingUp,
  },
  {
    name: 'Energy Bought',
    value: '1,230 kWh',
    change: '-5%',
    changeType: 'decrease' as const,
    icon: TrendingDown,
  },
  {
    name: 'Earnings',
    value: '$1,234',
    change: '+18%',
    changeType: 'increase' as const,
    icon: DollarSign,
  },
  {
    name: 'Active Trades',
    value: '12',
    change: '+3',
    changeType: 'increase' as const,
    icon: Activity,
  },
]

const recentTrades = [
  {
    id: 1,
    type: 'sell',
    amount: '50 kWh',
    price: '$0.12/kWh',
    peer: 'Solar Farm A',
    status: 'completed',
    time: '2 hours ago',
  },
  {
    id: 2,
    type: 'buy',
    amount: '25 kWh',
    price: '$0.10/kWh',
    peer: 'Wind Turbine B',
    status: 'pending',
    time: '4 hours ago',
  },
  {
    id: 3,
    type: 'sell',
    amount: '75 kWh',
    price: '$0.11/kWh',
    peer: 'Battery Storage C',
    status: 'completed',
    time: '6 hours ago',
  },
]

export default function Dashboard() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <div className="flex items-center space-x-2">
          <Sun className="h-5 w-5 text-yellow-500" />
          <span className="text-sm text-gray-600">Solar Production: 85%</span>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => {
          const Icon = stat.icon
          return (
            <Card key={stat.name} className="p-6">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <Icon className="h-6 w-6 text-gray-400" />
                </div>
                <div className="ml-4 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">
                      {stat.name}
                    </dt>
                    <dd className="flex items-baseline">
                      <div className="text-2xl font-semibold text-gray-900">
                        {stat.value}
                      </div>
                      <div className={`ml-2 flex items-baseline text-sm font-semibold ${
                        stat.changeType === 'increase' ? 'text-green-600' : 'text-red-600'
                      }`}>
                        {stat.change}
                      </div>
                    </dd>
                  </dl>
                </div>
              </div>
            </Card>
          )
        })}
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        {/* Energy Production */}
        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-medium text-gray-900">Energy Production</h3>
            <Battery className="h-5 w-5 text-green-500" />
          </div>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Solar Panels</span>
              <span className="text-sm font-medium">45 kWh</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div className="bg-yellow-500 h-2 rounded-full" style={{ width: '85%' }}></div>
            </div>
            
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Wind Turbine</span>
              <span className="text-sm font-medium">12 kWh</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div className="bg-blue-500 h-2 rounded-full" style={{ width: '45%' }}></div>
            </div>
            
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">Battery Storage</span>
              <span className="text-sm font-medium">28 kWh</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div className="bg-green-500 h-2 rounded-full" style={{ width: '70%' }}></div>
            </div>
          </div>
        </Card>

        {/* Recent Trades */}
        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-medium text-gray-900">Recent Trades</h3>
            <Users className="h-5 w-5 text-blue-500" />
          </div>
          <div className="space-y-4">
            {recentTrades.map((trade) => (
              <div key={trade.id} className="flex items-center justify-between py-2 border-b border-gray-100 last:border-b-0">
                <div className="flex items-center space-x-3">
                  <div className={`w-3 h-3 rounded-full ${
                    trade.type === 'sell' ? 'bg-green-500' : 'bg-blue-500'
                  }`} />
                  <div>
                    <p className="text-sm font-medium text-gray-900">
                      {trade.type === 'sell' ? 'Sold' : 'Bought'} {trade.amount}
                    </p>
                    <p className="text-xs text-gray-500">{trade.peer}</p>
                  </div>
                </div>
                <div className="text-right">
                  <Badge 
                    variant={trade.status === 'completed' ? 'success' : 'warning'}
                    className="mb-1"
                  >
                    {trade.status}
                  </Badge>
                  <p className="text-xs text-gray-500">{trade.time}</p>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </div>

      {/* Energy Grid Overview */}
      <Card className="p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium text-gray-900">Grid Overview</h3>
          <Zap className="h-5 w-5 text-purple-500" />
        </div>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
          <div className="text-center p-4 bg-green-50 rounded-lg">
            <div className="text-2xl font-bold text-green-600">127</div>
            <div className="text-sm text-green-600">Active Producers</div>
          </div>
          <div className="text-center p-4 bg-blue-50 rounded-lg">
            <div className="text-2xl font-bold text-blue-600">89</div>
            <div className="text-sm text-blue-600">Active Consumers</div>
          </div>
          <div className="text-center p-4 bg-purple-50 rounded-lg">
            <div className="text-2xl font-bold text-purple-600">$0.108</div>
            <div className="text-sm text-purple-600">Avg. Price/kWh</div>
          </div>
        </div>
      </Card>
    </div>
  )
}
