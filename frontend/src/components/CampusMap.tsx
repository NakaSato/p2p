import { useState } from 'react';
import { BuildingInfo, CampusMapProps } from '../types/campus';

// Building data based on actual UTCC campus layout
const buildings: BuildingInfo[] = [
  { id: 1, name: 'อาคาร 1', nameEn: 'Academic Resources Center', type: 'academic', x: 15, y: 85, width: 8, height: 6 },
  { id: 2, name: 'อาคาร 2', nameEn: 'Library Building', type: 'library', x: 25, y: 80, width: 8, height: 8 },
  { id: 3, name: 'อาคาร 3', nameEn: 'Sports Complex', type: 'facility', x: 35, y: 75, width: 10, height: 10 },
  { id: 4, name: 'อาคาร 4', nameEn: 'Learning and Activity Center', type: 'academic', x: 45, y: 70, width: 8, height: 8 },
  { id: 5, name: 'อาคาร 5', nameEn: 'Faculty of Business Administration', type: 'academic', x: 55, y: 65, width: 8, height: 8 },
  { id: 6, name: 'อาคาร 6', nameEn: 'Faculty of Accountancy', type: 'academic', x: 25, y: 60, width: 10, height: 8 },
  { id: 7, name: 'อาคาร 7', nameEn: 'Innovation & Entrepreneurship Center', type: 'facility', x: 15, y: 50, width: 8, height: 10 },
  { id: 8, name: 'อาคาร 8', nameEn: 'Graduate School Building', type: 'academic', x: 10, y: 40, width: 8, height: 8 },
  { id: 9, name: 'อาคาร 9', nameEn: 'Student Union', type: 'facility', x: 30, y: 45, width: 12, height: 8 },
  { id: 10, name: 'อาคาร 10', nameEn: 'School of Communication Arts', type: 'academic', x: 50, y: 40, width: 8, height: 8 },
  { id: 11, name: 'อาคาร 11', nameEn: 'Registrar\'s Office', type: 'admin', x: 20, y: 30, width: 8, height: 8 },
  { id: 12, name: 'อาคาร 12A', nameEn: 'International School of Management', type: 'academic', x: 35, y: 25, width: 10, height: 10 },
  { id: 14, name: 'อาคาร 14', nameEn: 'Faculty of Law', type: 'academic', x: 65, y: 30, width: 8, height: 8 },
  { id: 15, name: 'อาคาร 15', nameEn: 'Faculty of Economics', type: 'academic', x: 70, y: 40, width: 8, height: 8 },
  { id: 16, name: 'อาคาร 16', nameEn: 'Language Center', type: 'academic', x: 60, y: 55, width: 12, height: 8 },
  { id: 17, name: 'อาคาร 17', nameEn: 'Student Dormitory', type: 'facility', x: 75, y: 60, width: 8, height: 8 },
  { id: 18, name: 'อาคาร 18', nameEn: 'UTCC Main Tower', type: 'utcc', x: 45, y: 42, width: 16, height: 16 },
  { id: 19, name: 'อาคาร 19', nameEn: 'Kindergarten School (Lab School)', type: 'academic', x: 25, y: 15, width: 8, height: 6 },
  { id: 20, name: 'อาคาร 20', nameEn: 'University Printing House', type: 'facility', x: 40, y: 10, width: 8, height: 8 },
  { id: 21, name: 'อาคาร 21', nameEn: 'School of Engineering', type: 'academic', x: 55, y: 10, width: 8, height: 8 },
  { id: 22, name: 'อาคาร 22', nameEn: 'School of Humanities', type: 'academic', x: 70, y: 15, width: 8, height: 8 },
  { id: 23, name: 'อาคาร 23', nameEn: 'Co-Working Space', type: 'facility', x: 80, y: 25, width: 8, height: 8 },
  { id: 24, name: 'อาคาร 24', nameEn: 'Admin and President\'s Office', type: 'admin', x: 85, y: 45, width: 10, height: 10 },
  { id: 25, name: 'อาคาร 25', nameEn: 'Graduate School Building B', type: 'academic', x: 15, y: 65, width: 8, height: 8 }
];

const getBuildingColor = (type: string, isHovered: boolean, isSelected: boolean): string => {
  const colors = {
    academic: isSelected ? 'fill-blue-600' : isHovered ? 'fill-blue-400' : 'fill-blue-300',
    library: isSelected ? 'fill-purple-600' : isHovered ? 'fill-purple-400' : 'fill-purple-300',
    facility: isSelected ? 'fill-green-600' : isHovered ? 'fill-green-400' : 'fill-green-300',
    utcc: isSelected ? 'fill-teal-600' : isHovered ? 'fill-teal-400' : 'fill-teal-300',
    admin: isSelected ? 'fill-orange-600' : isHovered ? 'fill-orange-400' : 'fill-orange-300'
  };
  return colors[type as keyof typeof colors] || colors.academic;
};

const CampusMap = ({ onBuildingSelect, selectedBuildingId }: CampusMapProps) => {
  const [hoveredBuilding, setHoveredBuilding] = useState<number | null>(null);

  const handleBuildingClick = (building: BuildingInfo) => {
    onBuildingSelect?.(building);
  };

  return (
    <div className="w-full max-w-6xl mx-auto bg-white rounded-lg shadow-lg overflow-hidden">
      <div className="bg-gradient-to-r from-blue-600 to-teal-600 text-white p-4">
        <h2 className="text-2xl font-bold">UTCC Campus Map</h2>
        <p className="text-blue-100">มหาวิทยาลัยหอการค้าไทย - Interactive Building Diagram</p>
      </div>
      
      <div className="p-4">
        <div className="relative w-full" style={{ paddingBottom: '60%' }}>
          <svg
            className="absolute inset-0 w-full h-full"
            viewBox="0 0 100 100"
            preserveAspectRatio="xMidYMid meet"
          >
            {/* Campus background */}
            <defs>
              <pattern id="grass" patternUnits="userSpaceOnUse" width="4" height="4">
                <rect width="4" height="4" fill="#10b981" />
                <circle cx="2" cy="2" r="0.3" fill="#065f46" opacity="0.3" />
              </pattern>
              <linearGradient id="roadGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stopColor="#6b7280" />
                <stop offset="50%" stopColor="#9ca3af" />
                <stop offset="100%" stopColor="#6b7280" />
              </linearGradient>
            </defs>
            
            {/* Campus ground */}
            <rect x="0" y="0" width="100" height="100" fill="url(#grass)" />
            
            {/* Main roads */}
            <path d="M 0 50 Q 25 45 50 50 Q 75 55 100 50" stroke="url(#roadGradient)" strokeWidth="3" fill="none" />
            <path d="M 20 0 Q 22 25 20 50 Q 18 75 20 100" stroke="url(#roadGradient)" strokeWidth="2" fill="none" />
            <path d="M 50 0 Q 52 25 50 50 Q 48 75 50 100" stroke="url(#roadGradient)" strokeWidth="2" fill="none" />
            <path d="M 80 0 Q 82 25 80 50 Q 78 75 80 100" stroke="url(#roadGradient)" strokeWidth="2" fill="none" />
            
            {/* Campus center area - will be covered by Building 18 (UTCC Main Tower) */}
            
            {/* Buildings */}
            {buildings.map((building) => {
              const isHovered = hoveredBuilding === building.id;
              const isSelected = selectedBuildingId === building.id;
              
              return (
                <g key={building.id}>
                  <rect
                    x={building.x}
                    y={building.y}
                    width={building.width}
                    height={building.height}
                    className={`${getBuildingColor(building.type, isHovered, isSelected)} stroke-gray-600 stroke-1 cursor-pointer transition-all duration-200`}
                    rx="0.5"
                    onMouseEnter={() => setHoveredBuilding(building.id)}
                    onMouseLeave={() => setHoveredBuilding(null)}
                    onClick={() => handleBuildingClick(building)}
                    style={{
                      filter: isHovered ? 'drop-shadow(0 4px 8px rgba(0,0,0,0.3))' : 'none',
                      transform: isSelected ? 'scale(1.05)' : 'scale(1)',
                      transformOrigin: 'center'
                    }}
                  />
                  
                  {/* Building number */}
                  <circle
                    cx={building.x + building.width / 2}
                    cy={building.y - 1}
                    r="1.5"
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
                  
                  {/* Building label on hover */}
                  {isHovered && (
                    <g>
                      <rect
                        x={building.x + building.width / 2 - 6}
                        y={building.y + building.height + 1}
                        width="12"
                        height="3"
                        className="fill-black fill-opacity-80"
                        rx="0.5"
                      />
                      <text
                        x={building.x + building.width / 2}
                        y={building.y + building.height + 2.5}
                        textAnchor="middle"
                        className="text-xs font-semibold fill-white"
                      >
                        {building.name}
                      </text>
                    </g>
                  )}
                </g>
              );
            })}
            
            {/* Trees and landscaping */}
            {[
              { x: 10, y: 10 }, { x: 85, y: 15 }, { x: 15, y: 35 }, { x: 65, y: 25 },
              { x: 30, y: 85 }, { x: 75, y: 85 }, { x: 5, y: 70 }, { x: 90, y: 70 }
            ].map((tree, i) => (
              <circle
                key={i}
                cx={tree.x}
                cy={tree.y}
                r="1.5"
                className="fill-green-500 opacity-60"
              />
            ))}
          </svg>
        </div>
        
        {/* Legend */}
        <div className="mt-4 flex flex-wrap gap-4 text-sm">
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-blue-300 rounded"></div>
            <span>Academic Buildings</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-purple-300 rounded"></div>
            <span>Library</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-green-300 rounded"></div>
            <span>Facilities & Services</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-orange-300 rounded"></div>
            <span>Administration</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-teal-400 rounded"></div>
            <span>Main Tower</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CampusMap;
