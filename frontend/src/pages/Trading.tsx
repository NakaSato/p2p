import { useState } from 'react'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { 
  TrendingUp, 
  TrendingDown, 
  RefreshCw,
  Filter,
  Search,
  Zap,
  Users,
  Clock
} from 'lucide-react'

interface Trade {
  id: string
  type: 'buy' | 'sell'
  amount: number
  price: number
  peer: string
  status: 'active' | 'completed' | 'cancelled'
  timestamp: string
  renewable: boolean
}

const mockTrades: Trade[] = [
  {
    id: '1',
    type: 'sell',
    amount: 100,
    price: 0.12,
    peer: 'Solar Farm Alpha',
    status: 'active',
    timestamp: '2024-01-15T10:30:00Z',
    renewable: true,
  },
  {
    id: '2',
    type: 'buy',
    amount: 50,
    price: 0.10,
    peer: 'Wind Turbine Beta',
    status: 'completed',
    timestamp: '2024-01-15T09:15:00Z',
    renewable: true,
  },
  {
    id: '3',
    type: 'sell',
    amount: 75,
    price: 0.11,
    peer: 'Battery Storage Gamma',
    status: 'active',
    timestamp: '2024-01-15T08:45:00Z',
    renewable: false,
  },
  {
    id: '4',
    type: 'buy',
    amount: 25,
    price: 0.09,
    peer: 'Hydroelectric Delta',
    status: 'cancelled',
    timestamp: '2024-01-15T07:20:00Z',
    renewable: true,
  },
]

export default function Trading() {
  const [selectedType, setSelectedType] = useState<'all' | 'buy' | 'sell'>('all')
  const [selectedStatus, setSelectedStatus] = useState<'all' | 'active' | 'completed' | 'cancelled'>('all')
  const [searchTerm, setSearchTerm] = useState('')

  const filteredTrades = mockTrades.filter(trade => {
    const matchesType = selectedType === 'all' || trade.type === selectedType
    const matchesStatus = selectedStatus === 'all' || trade.status === selectedStatus
    const matchesSearch = trade.peer.toLowerCase().includes(searchTerm.toLowerCase())
    return matchesType && matchesStatus && matchesSearch
  })

  const formatTime = (timestamp: string) => {
    return new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }

  const formatDate = (timestamp: string) => {
    return new Date(timestamp).toLocaleDateString()
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">Energy Trading</h1>
        <button className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
          <Zap className="h-4 w-4 mr-2" />
          New Trade
        </button>
      </div>

      {/* Market Overview */}
      <div className="grid grid-cols-1 gap-5 sm:grid-cols-3">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <TrendingUp className="h-6 w-6 text-green-500" />
            </div>
            <div className="ml-4">
              <div className="text-sm font-medium text-gray-500">Market Price</div>
              <div className="text-2xl font-semibold text-gray-900">$0.108/kWh</div>
              <div className="text-sm text-green-600">+2.4% from yesterday</div>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <Users className="h-6 w-6 text-blue-500" />
            </div>
            <div className="ml-4">
              <div className="text-sm font-medium text-gray-500">Active Traders</div>
              <div className="text-2xl font-semibold text-gray-900">216</div>
              <div className="text-sm text-blue-600">12 new this hour</div>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <Clock className="h-6 w-6 text-purple-500" />
            </div>
            <div className="ml-4">
              <div className="text-sm font-medium text-gray-500">Avg. Settlement</div>
              <div className="text-2xl font-semibold text-gray-900">4.2 min</div>
              <div className="text-sm text-purple-600">Fast network</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Filters and Search */}
      <Card className="p-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <Filter className="h-4 w-4 text-gray-400" />
              <select
                value={selectedType}
                onChange={(e) => setSelectedType(e.target.value as 'all' | 'buy' | 'sell')}
                className="border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              >
                <option value="all">All Types</option>
                <option value="buy">Buy Orders</option>
                <option value="sell">Sell Orders</option>
              </select>
            </div>

            <select
              value={selectedStatus}
              onChange={(e) => setSelectedStatus(e.target.value as 'all' | 'active' | 'completed' | 'cancelled')}
              className="border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            >
              <option value="all">All Status</option>
              <option value="active">Active</option>
              <option value="completed">Completed</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>

          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <Search className="h-4 w-4 text-gray-400" />
            </div>
            <input
              type="text"
              placeholder="Search peers..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
            />
          </div>
        </div>
      </Card>

      {/* Trades List */}
      <Card className="overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-medium text-gray-900">Recent Trades</h3>
            <button className="inline-flex items-center text-sm text-gray-500 hover:text-gray-700">
              <RefreshCw className="h-4 w-4 mr-1" />
              Refresh
            </button>
          </div>
        </div>

        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Type
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Amount
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Price
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Peer
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Time
                </th>
                <th className="relative px-6 py-3">
                  <span className="sr-only">Actions</span>
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {filteredTrades.map((trade) => (
                <tr key={trade.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex items-center">
                      {trade.type === 'sell' ? (
                        <TrendingUp className="h-4 w-4 text-green-500 mr-2" />
                      ) : (
                        <TrendingDown className="h-4 w-4 text-blue-500 mr-2" />
                      )}
                      <span className={`text-sm font-medium ${
                        trade.type === 'sell' ? 'text-green-600' : 'text-blue-600'
                      }`}>
                        {trade.type.toUpperCase()}
                      </span>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {trade.amount} kWh
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    ${trade.price.toFixed(3)}/kWh
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex items-center">
                      <span className="text-sm text-gray-900">{trade.peer}</span>
                      {trade.renewable && (
                        <span className="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                          Renewable
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <Badge 
                      variant={
                        trade.status === 'completed' ? 'success' : 
                        trade.status === 'active' ? 'default' : 
                        'error'
                      }
                    >
                      {trade.status}
                    </Badge>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    <div>
                      <div>{formatTime(trade.timestamp)}</div>
                      <div className="text-xs">{formatDate(trade.timestamp)}</div>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <button className="text-primary-600 hover:text-primary-900">
                      View
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {filteredTrades.length === 0 && (
          <div className="px-6 py-12 text-center">
            <div className="text-gray-500">No trades found matching your criteria.</div>
          </div>
        )}
      </Card>
    </div>
  )
}
