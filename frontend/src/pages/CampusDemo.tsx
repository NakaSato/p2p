import { useState } from 'react';
import { CampusMap, EmbeddedCampusMap, BuildingDetail } from '../components';
import { BuildingInfo } from '../types/campus';

const CampusDemo = () => {
  const [selectedBuilding, setSelectedBuilding] = useState<BuildingInfo | null>(null);
  const [activeTab, setActiveTab] = useState<'full' | 'embedded'>('full');

  const handleBuildingSelect = (building: BuildingInfo) => {
    setSelectedBuilding(building);
  };

  return (
    <div className="min-h-screen bg-gray-100 p-6">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">
            Interactive Campus Map Demo
          </h1>
          <p className="text-gray-600 mb-6">
            This demo shows how to convert building diagrams into interactive web components
          </p>
          
          {/* Tab Navigation */}
          <div className="flex justify-center mb-6">
            <div className="bg-white rounded-lg p-1 shadow-md">
              <button
                className={`px-4 py-2 rounded-md transition-colors ${
                  activeTab === 'full' 
                    ? 'bg-blue-600 text-white' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setActiveTab('full')}
              >
                Full Interactive Map
              </button>
              <button
                className={`px-4 py-2 rounded-md transition-colors ${
                  activeTab === 'embedded' 
                    ? 'bg-blue-600 text-white' 
                    : 'text-gray-600 hover:text-gray-900'
                }`}
                onClick={() => setActiveTab('embedded')}
              >
                Embedded Widget
              </button>
            </div>
          </div>
        </div>

        {/* Full Map View */}
        {activeTab === 'full' && (
          <div className="space-y-6">
            <CampusMap 
              onBuildingSelect={handleBuildingSelect}
              selectedBuildingId={selectedBuilding?.id}
            />
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-white p-4 rounded-lg shadow-lg">
                <h3 className="font-semibold text-gray-900 mb-3">Features</h3>
                <ul className="text-sm text-gray-600 space-y-1">
                  <li>✓ Interactive SVG graphics</li>
                  <li>✓ Hover and click effects</li>
                  <li>✓ Responsive design</li>
                  <li>✓ Building detail modals</li>
                  <li>✓ TypeScript support</li>
                </ul>
              </div>
              
              <div className="bg-white p-4 rounded-lg shadow-lg">
                <h3 className="font-semibold text-gray-900 mb-3">Technologies</h3>
                <ul className="text-sm text-gray-600 space-y-1">
                  <li>• React 18+ with hooks</li>
                  <li>• TypeScript for type safety</li>
                  <li>• Tailwind CSS styling</li>
                  <li>• SVG for scalable graphics</li>
                  <li>• Lucide React icons</li>
                </ul>
              </div>
              
              <div className="bg-white p-4 rounded-lg shadow-lg">
                <h3 className="font-semibold text-gray-900 mb-3">Use Cases</h3>
                <ul className="text-sm text-gray-600 space-y-1">
                  <li>• Campus navigation</li>
                  <li>• Event location finder</li>
                  <li>• Facility management</li>
                  <li>• Virtual tours</li>
                  <li>• Mobile applications</li>
                </ul>
              </div>
            </div>
          </div>
        )}

        {/* Embedded Widget View */}
        {activeTab === 'embedded' && (
          <div className="space-y-6">
            <div className="text-center">
              <h2 className="text-xl font-semibold text-gray-900 mb-4">
                Embedded Campus Widget
              </h2>
              <p className="text-gray-600 mb-6">
                A compact version perfect for sidebars, dashboards, or mobile apps
              </p>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {/* Different sizes and configurations */}
              <div>
                <h3 className="font-medium text-gray-900 mb-2">Standard Size</h3>
                <EmbeddedCampusMap 
                  onBuildingClick={handleBuildingSelect}
                  className="w-full"
                />
              </div>
              
              <div>
                <h3 className="font-medium text-gray-900 mb-2">Compact (No Labels)</h3>
                <EmbeddedCampusMap 
                  onBuildingClick={handleBuildingSelect}
                  showLabels={false}
                  className="w-full"
                />
              </div>
              
              <div>
                <h3 className="font-medium text-gray-900 mb-2">Dashboard Widget</h3>
                <div className="bg-white p-4 rounded-lg shadow-lg">
                  <div className="flex justify-between items-center mb-3">
                    <h4 className="font-semibold">Campus Overview</h4>
                    <span className="text-xs text-gray-500">Live</span>
                  </div>
                  <EmbeddedCampusMap 
                    onBuildingClick={handleBuildingSelect}
                    showLabels={false}
                    className="w-full mb-3"
                  />
                  <div className="text-xs text-gray-600">
                    <p>28 Buildings • 15,000+ Students</p>
                  </div>
                </div>
              </div>
            </div>
            
            <div className="bg-blue-50 p-4 rounded-lg">
              <h3 className="font-semibold text-blue-900 mb-2">Integration Example</h3>
              <pre className="text-xs text-blue-800 bg-white p-3 rounded overflow-x-auto">
{`import { EmbeddedCampusMap } from './components';

function Dashboard() {
  return (
    <div className="dashboard">
      <EmbeddedCampusMap 
        onBuildingClick={(building) => {
          console.log('Selected:', building.name);
        }}
        showLabels={true}
        className="w-full"
      />
    </div>
  );
}`}
              </pre>
            </div>
          </div>
        )}

        {/* Building Detail Modal */}
        <BuildingDetail
          building={selectedBuilding}
          onClose={() => setSelectedBuilding(null)}
        />
      </div>
    </div>
  );
};

export default CampusDemo;
