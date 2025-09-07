import { useState } from 'react'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { 
  User, 
  Edit2, 
  MapPin, 
  Phone, 
  Mail, 
  Calendar,
  Zap,
  TrendingUp,
  Award,
  Shield
} from 'lucide-react'

interface UserProfile {
  id: string
  name: string
  email: string
  phone: string
  location: string
  joinDate: string
  energyType: string[]
  capacity: number
  verified: boolean
  rating: number
  totalTrades: number
  successRate: number
  carbonSaved: number
}

const userProfile: UserProfile = {
  id: 'user_123',
  name: 'John Smith',
  email: 'john.smith@example.com',
  phone: '+1 (555) 123-4567',
  location: 'San Francisco, CA',
  joinDate: '2023-06-15',
  energyType: ['Solar', 'Wind'],
  capacity: 150,
  verified: true,
  rating: 4.8,
  totalTrades: 247,
  successRate: 98.4,
  carbonSaved: 2450
}

const recentActivity = [
  {
    id: 1,
    type: 'trade',
    description: 'Sold 50 kWh to Solar Farm Alpha',
    amount: '$6.00',
    timestamp: '2 hours ago',
    status: 'completed'
  },
  {
    id: 2,
    type: 'verification',
    description: 'Solar panel capacity verified',
    amount: null,
    timestamp: '1 day ago',
    status: 'completed'
  },
  {
    id: 3,
    type: 'trade',
    description: 'Bought 25 kWh from Wind Turbine Beta',
    amount: '$2.50',
    timestamp: '3 days ago',
    status: 'completed'
  }
]

export default function Profile() {
  const [isEditing, setIsEditing] = useState(false)

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">Profile</h1>
        <button 
          onClick={() => setIsEditing(!isEditing)}
          className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
        >
          <Edit2 className="h-4 w-4 mr-2" />
          {isEditing ? 'Save Changes' : 'Edit Profile'}
        </button>
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        {/* Profile Information */}
        <div className="lg:col-span-2">
          <Card className="p-6">
            <div className="flex items-center space-x-6 mb-6">
              <div className="h-24 w-24 rounded-full bg-primary-100 flex items-center justify-center">
                <User className="h-12 w-12 text-primary-600" />
              </div>
              <div className="flex-1">
                <div className="flex items-center space-x-2 mb-2">
                  <h2 className="text-2xl font-bold text-gray-900">{userProfile.name}</h2>
                  {userProfile.verified && (
                    <Badge variant="success" className="flex items-center">
                      <Shield className="h-3 w-3 mr-1" />
                      Verified
                    </Badge>
                  )}
                </div>
                <div className="flex items-center space-x-4 text-sm text-gray-500">
                  <div className="flex items-center">
                    <Calendar className="h-4 w-4 mr-1" />
                    Joined {new Date(userProfile.joinDate).toLocaleDateString()}
                  </div>
                  <div className="flex items-center">
                    <Award className="h-4 w-4 mr-1" />
                    {userProfile.rating} ⭐ Rating
                  </div>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Email Address
                </label>
                {isEditing ? (
                  <input
                    type="email"
                    value={userProfile.email}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                  />
                ) : (
                  <div className="flex items-center text-gray-900">
                    <Mail className="h-4 w-4 mr-2 text-gray-400" />
                    {userProfile.email}
                  </div>
                )}
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Phone Number
                </label>
                {isEditing ? (
                  <input
                    type="tel"
                    value={userProfile.phone}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                  />
                ) : (
                  <div className="flex items-center text-gray-900">
                    <Phone className="h-4 w-4 mr-2 text-gray-400" />
                    {userProfile.phone}
                  </div>
                )}
              </div>

              <div className="sm:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Location
                </label>
                {isEditing ? (
                  <input
                    type="text"
                    value={userProfile.location}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
                  />
                ) : (
                  <div className="flex items-center text-gray-900">
                    <MapPin className="h-4 w-4 mr-2 text-gray-400" />
                    {userProfile.location}
                  </div>
                )}
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Energy Sources
                </label>
                <div className="flex flex-wrap gap-2">
                  {userProfile.energyType.map((type) => (
                    <Badge key={type} variant="default">
                      {type}
                    </Badge>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Production Capacity
                </label>
                <div className="flex items-center text-gray-900">
                  <Zap className="h-4 w-4 mr-2 text-gray-400" />
                  {userProfile.capacity} kWh/day
                </div>
              </div>
            </div>
          </Card>
        </div>

        {/* Stats Sidebar */}
        <div className="space-y-6">
          {/* Trading Stats */}
          <Card className="p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Trading Stats</h3>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600">Total Trades</span>
                <span className="text-sm font-medium text-gray-900">{userProfile.totalTrades}</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600">Success Rate</span>
                <span className="text-sm font-medium text-green-600">{userProfile.successRate}%</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600">Carbon Saved</span>
                <span className="text-sm font-medium text-green-600">{userProfile.carbonSaved} kg CO₂</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600">Rating</span>
                <span className="text-sm font-medium text-yellow-600">{userProfile.rating} ⭐</span>
              </div>
            </div>
          </Card>

          {/* Recent Activity */}
          <Card className="p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Recent Activity</h3>
            <div className="space-y-4">
              {recentActivity.map((activity) => (
                <div key={activity.id} className="flex items-start space-x-3">
                  <div className={`flex-shrink-0 w-2 h-2 rounded-full mt-2 ${
                    activity.type === 'trade' ? 'bg-blue-500' : 'bg-green-500'
                  }`} />
                  <div className="flex-1 min-w-0">
                    <p className="text-sm text-gray-900">{activity.description}</p>
                    <div className="flex items-center justify-between mt-1">
                      <p className="text-xs text-gray-500">{activity.timestamp}</p>
                      {activity.amount && (
                        <p className="text-xs font-medium text-green-600">{activity.amount}</p>
                      )}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </Card>

          {/* Achievements */}
          <Card className="p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Achievements</h3>
            <div className="grid grid-cols-2 gap-3">
              <div className="text-center p-3 bg-yellow-50 rounded-lg">
                <Award className="h-6 w-6 text-yellow-600 mx-auto mb-1" />
                <div className="text-xs font-medium text-yellow-600">Top Trader</div>
              </div>
              <div className="text-center p-3 bg-green-50 rounded-lg">
                <Zap className="h-6 w-6 text-green-600 mx-auto mb-1" />
                <div className="text-xs font-medium text-green-600">Green Pioneer</div>
              </div>
              <div className="text-center p-3 bg-blue-50 rounded-lg">
                <TrendingUp className="h-6 w-6 text-blue-600 mx-auto mb-1" />
                <div className="text-xs font-medium text-blue-600">Consistent Seller</div>
              </div>
              <div className="text-center p-3 bg-purple-50 rounded-lg">
                <Shield className="h-6 w-6 text-purple-600 mx-auto mb-1" />
                <div className="text-xs font-medium text-purple-600">Verified Producer</div>
              </div>
            </div>
          </Card>
        </div>
      </div>
    </div>
  )
}
