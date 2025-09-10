import { useState } from 'react';
import CampusMap from '../components/CampusMap';
import BuildingDetail from '../components/BuildingDetail';
import { BuildingInfo } from '../types/campus';

const CampusMapPage = () => {
  const [selectedBuilding, setSelectedBuilding] = useState<BuildingInfo | null>(null);

  const handleBuildingSelect = (building: BuildingInfo) => {
    // Add more details to buildings when selected
    const enhancedBuilding: BuildingInfo = {
      ...building,
      description: getBuildingDescription(building.id),
      facilities: getBuildingFacilities(building.id),
      floors: getBuildingFloors(building.id)
    };
    setSelectedBuilding(enhancedBuilding);
  };

  const handleCloseDetail = () => {
    setSelectedBuilding(null);
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="container mx-auto px-4">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">
            UTCC Interactive Campus Map
          </h1>
          <p className="text-lg text-gray-600 max-w-2xl mx-auto">
            Explore the University of Thai Chamber of Commerce campus. 
            Click on any building to learn more about its facilities and departments.
          </p>
        </div>

        <CampusMap 
          onBuildingSelect={handleBuildingSelect}
          selectedBuildingId={selectedBuilding?.id}
        />

        <div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-white p-6 rounded-lg shadow-lg">
            <h3 className="text-lg font-semibold mb-3 text-blue-600">Quick Navigation</h3>
            <ul className="space-y-2 text-sm">
              <li>• Click on building numbers to view details</li>
              <li>• Hover over buildings for quick info</li>
              <li>• Different colors represent building types</li>
              <li>• Interactive legend shows building categories</li>
            </ul>
          </div>

          <div className="bg-white p-6 rounded-lg shadow-lg">
            <h3 className="text-lg font-semibold mb-3 text-green-600">Campus Features</h3>
            <ul className="space-y-2 text-sm">
              <li>• 28 Academic and Administrative Buildings</li>
              <li>• Central Library (Building 24)</li>
              <li>• Modern Facilities and Labs</li>
              <li>• Green Spaces and Recreation Areas</li>
            </ul>
          </div>

          <div className="bg-white p-6 rounded-lg shadow-lg">
            <h3 className="text-lg font-semibold mb-3 text-purple-600">Technology Stack</h3>
            <ul className="space-y-2 text-sm">
              <li>• React + TypeScript</li>
              <li>• SVG-based Interactive Graphics</li>
              <li>• Tailwind CSS Styling</li>
              <li>• Responsive Design</li>
            </ul>
          </div>
        </div>
      </div>

      <BuildingDetail
        building={selectedBuilding}
        onClose={handleCloseDetail}
      />
    </div>
  );
};

// Helper functions to provide building details
const getBuildingDescription = (buildingId: number): string => {
  const descriptions: Record<number, string> = {
    1: "Central hub for academic resources, student support services, and learning materials.",
    2: "Main library facility with extensive collections, digital resources, and study spaces.",
    3: "Complete sports and recreation complex with gymnasium, courts, and fitness facilities.",
    4: "Multi-purpose center for learning activities, workshops, and student engagement programs.",
    5: "Home to the Faculty of Business Administration with classrooms, offices, and meeting rooms.",
    6: "Faculty of Accountancy building with specialized labs, classrooms, and faculty offices.",
    7: "Innovation and Entrepreneurship Center fostering startup culture and business incubation.",
    8: "Graduate School building with research facilities, seminar rooms, and administrative offices.",
    9: "Student Union building with dining facilities, recreational areas, and student services.",
    10: "School of Communication Arts with media labs, studios, and production facilities.",
    11: "Registrar's Office handling student records, enrollment, and academic administration.",
    12: "International School of Management with global programs and international student services.",
    14: "Faculty of Law with moot courts, law library, and legal clinic facilities.",
    15: "Faculty of Economics with research centers and economic analysis laboratories.",
    16: "Language Center offering language courses and cultural exchange programs.",
    17: "On-campus student dormitory providing residential accommodation for students.",
    18: "UTCC Main Tower - the iconic central building housing key administrative offices.",
    19: "Laboratory school and kindergarten providing early childhood education services.",
    20: "University printing house handling publications and printing services.",
    21: "School of Engineering with laboratories, workshops, and technical facilities.",
    22: "School of Humanities with classrooms, research centers, and cultural programs.",
    23: "Modern co-working space for students, faculty, and community collaboration.",
    24: "Administration building housing the President's Office and key administrative units.",
    25: "Additional graduate school building with advanced research and study facilities."
  };
  
  return descriptions[buildingId] || "Academic building with specialized facilities and services.";
};

const getBuildingFacilities = (buildingId: number): string[] => {
  const facilities: Record<number, string[]> = {
    1: ["Academic Resource Center", "Study Areas", "Student Support Services", "Learning Commons"],
    2: ["Digital Library", "Book Collections", "Study Rooms", "Research Databases", "Computer Labs"],
    3: ["Gymnasium", "Basketball Courts", "Fitness Center", "Swimming Pool", "Sports Equipment"],
    4: ["Activity Halls", "Workshop Rooms", "Event Spaces", "Presentation Areas", "Group Study Rooms"],
    5: ["Business Classrooms", "Case Study Rooms", "Faculty Offices", "Meeting Rooms", "Student Lounges"],
    6: ["Accounting Labs", "Computer Labs", "Seminar Rooms", "Faculty Offices", "Research Centers"],
    7: ["Startup Incubators", "Co-working Spaces", "Innovation Labs", "Presentation Rooms", "Mentoring Areas"],
    8: ["Research Labs", "Graduate Offices", "Seminar Rooms", "Thesis Defense Rooms", "Faculty Lounges"],
    9: ["Food Court", "Student Activities Center", "Meeting Rooms", "Recreation Areas", "Information Desk"],
    10: ["Media Production Studios", "Computer Labs", "Radio Station", "TV Studio", "Editing Suites"],
    11: ["Registration Services", "Student Records", "Administrative Offices", "Waiting Areas", "Information Counter"],
    12: ["International Classrooms", "Language Labs", "Cultural Centers", "Exchange Offices", "Global Programs"],
    14: ["Moot Court", "Law Library", "Legal Clinic", "Faculty Offices", "Student Study Areas"],
    15: ["Economics Labs", "Research Centers", "Data Analysis Rooms", "Faculty Offices", "Seminar Rooms"],
    16: ["Language Classrooms", "Multimedia Labs", "Cultural Exchange Center", "Testing Centers", "Faculty Offices"],
    17: ["Student Rooms", "Common Areas", "Laundry Facilities", "Study Lounges", "Recreation Rooms"],
    18: ["Executive Offices", "Conference Rooms", "Reception Areas", "Administrative Services", "Meeting Halls"],
    19: ["Kindergarten Classrooms", "Play Areas", "Learning Centers", "Teacher Offices", "Parent Meeting Rooms"],
    20: ["Printing Services", "Publication Center", "Design Studios", "Storage Areas", "Quality Control"],
    21: ["Engineering Labs", "Workshops", "Computer-Aided Design Labs", "Project Rooms", "Faculty Offices"],
    22: ["Liberal Arts Classrooms", "Research Centers", "Cultural Programs Office", "Faculty Lounges", "Study Areas"],
    23: ["Co-working Desks", "Meeting Rooms", "Collaboration Spaces", "Event Areas", "Networking Lounges"],
    24: ["President's Office", "Administrative Offices", "Board Rooms", "Reception Areas", "Executive Services"],
    25: ["Advanced Research Labs", "Graduate Student Offices", "Seminar Rooms", "Library Resources", "Study Spaces"]
  };
  
  return facilities[buildingId] || ["Classrooms", "Faculty Offices", "Study Areas"];
};

const getBuildingFloors = (buildingId: number): number => {
  const floors: Record<number, number> = {
    18: 20, // UTCC Main Tower - tallest building
    2: 8,   // Library Building
    5: 7,   // Faculty of Business Administration
    6: 6,   // Faculty of Accountancy
    8: 6,   // Graduate School Building
    12: 5,  // International School of Management
    3: 5,   // Sports Complex
    7: 4,   // Innovation & Entrepreneurship Center
    9: 4,   // Student Union
    10: 4,  // School of Communication Arts
    14: 4,  // Faculty of Law
    15: 4,  // Faculty of Economics
    21: 4,  // School of Engineering
    24: 6,  // Admin and President's Office
    25: 5,  // Graduate School Building B
    1: 3, 4: 3, 11: 3, 16: 3, 17: 6, 19: 2, 20: 2, 22: 3, 23: 3
  };
  
  return floors[buildingId] || 3;
};

export default CampusMapPage;
