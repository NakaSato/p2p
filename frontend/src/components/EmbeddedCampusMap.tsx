import { useState } from 'react';
import { BuildingInfo } from '../types/campus';

// Simplified building data for smaller displays
const buildings: BuildingInfo[] = [
  { id: 1, name: 'อาคาร 1', nameEn: 'Academic Resources Center', type: 'academic', x: 20, y: 80, width: 6, height: 4 },
  { id: 2, name: 'อาคาร 2', nameEn: 'Library Building', type: 'library', x: 40, y: 45, width: 12, height: 10 },
  { id: 9, name: 'อาคาร 9', nameEn: 'Student Union', type: 'facility', x: 25, y: 60, width: 8, height: 6 },
  { id: 18, name: 'อาคาร 18', nameEn: 'UTCC Main Tower', type: 'utcc', x: 45, y: 35, width: 10, height: 12 },
  { id: 5, name: 'อาคาร 5', nameEn: 'Faculty of Business Administration', type: 'academic', x: 65, y: 55, width: 8, height: 6 }
];

const getBuildingColor = (type: string, isHovered: boolean): string => {
  const colors = {
    academic: isHovered ? 'fill-blue-400' : 'fill-blue-300',
    library: isHovered ? 'fill-purple-400' : 'fill-purple-300',
    facility: isHovered ? 'fill-green-400' : 'fill-green-300',
    utcc: isHovered ? 'fill-teal-400' : 'fill-teal-300',
    admin: isHovered ? 'fill-orange-400' : 'fill-orange-300'
  };
  return colors[type as keyof typeof colors] || colors.academic;
};

interface EmbeddedCampusMapProps {
  className?: string;
  showLabels?: boolean;
  onBuildingClick?: (building: BuildingInfo) => void;
}

const EmbeddedCampusMap = ({ className = '', showLabels = true, onBuildingClick }: EmbeddedCampusMapProps) => {
  const [hoveredBuilding, setHoveredBuilding] = useState<number | null>(null);

  const handleBuildingClick = (building: BuildingInfo) => {
    onBuildingClick?.(building);
  };

  return (
    <div className={`bg-white rounded-lg shadow-md overflow-hidden ${className}`}>
      {showLabels && (
        <div className="bg-gradient-to-r from-blue-500 to-teal-500 text-white p-3">
          <h3 className="text-lg font-semibold">UTCC Campus</h3>
        </div>
      )}
      
      <div className="p-3">
        <div className="relative w-full" style={{ paddingBottom: '50%' }}>
          <svg
            className="absolute inset-0 w-full h-full"
            viewBox="0 0 100 100"
            preserveAspectRatio="xMidYMid meet"
          >
            {/* Campus background */}
            <rect x="0" y="0" width="100" height="100" fill="#e5f3f0" />
            
            {/* Campus pathways */}
            <path d="M 0 50 Q 25 45 50 50 Q 75 55 100 50" stroke="#9ca3af" strokeWidth="2" fill="none" />
            
            {/* Buildings */}
            {buildings.map((building) => {
              const isHovered = hoveredBuilding === building.id;
              
              return (
                <g key={building.id}>
                  <rect
                    x={building.x}
                    y={building.y}
                    width={building.width}
                    height={building.height}
                    className={`${getBuildingColor(building.type, isHovered)} stroke-gray-600 stroke-1 cursor-pointer transition-all duration-200`}
                    rx="0.5"
                    onMouseEnter={() => setHoveredBuilding(building.id)}
                    onMouseLeave={() => setHoveredBuilding(null)}
                    onClick={() => handleBuildingClick(building)}
                  />
                  
                  <circle
                    cx={building.x + building.width / 2}
                    cy={building.y - 1}
                    r="1.2"
                    className="fill-yellow-400 stroke-yellow-600 stroke-1 cursor-pointer"
                    onMouseEnter={() => setHoveredBuilding(building.id)}
                    onMouseLeave={() => setHoveredBuilding(null)}
                    onClick={() => handleBuildingClick(building)}
                  />
                  <text
                    x={building.x + building.width / 2}
                    y={building.y - 0.3}
                    textAnchor="middle"
                    className="text-xs font-bold fill-black cursor-pointer pointer-events-none"
                  >
                    {building.id}
                  </text>
                </g>
              );
            })}
          </svg>
        </div>
        
        {showLabels && (
          <div className="mt-2 flex flex-wrap gap-2 text-xs">
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-blue-300 rounded"></div>
              <span>Academic</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-purple-300 rounded"></div>
              <span>Library</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 bg-green-300 rounded"></div>
              <span>Facilities</span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default EmbeddedCampusMap;
