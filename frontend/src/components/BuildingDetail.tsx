import { X } from 'lucide-react';
import { BuildingDetailProps } from '../types/campus';

const BuildingDetail = ({ building, onClose }: BuildingDetailProps) => {
  if (!building) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4 relative" onClick={(e) => e.stopPropagation()}>
        <button
          onClick={onClose}
          className="absolute top-4 right-4 text-gray-500 hover:text-gray-700"
        >
          <X size={24} />
        </button>
        
        <div className="mb-4">
          <h3 className="text-xl font-bold text-gray-900 mb-2">
            {building.name}
          </h3>
          <p className="text-gray-600 mb-1">{building.nameEn}</p>
          <span className="inline-block px-3 py-1 bg-blue-100 text-blue-800 text-sm rounded-full">
            {building.type === 'academic' && 'Academic Building'}
            {building.type === 'library' && 'Library'}
            {building.type === 'facility' && 'Facility'}
            {building.type === 'utcc' && 'Main Building'}
            {building.type === 'admin' && 'Administration'}
          </span>
        </div>
        
        {building.description && (
          <div className="mb-4">
            <h4 className="font-semibold text-gray-900 mb-2">Description</h4>
            <p className="text-gray-700">{building.description}</p>
          </div>
        )}
        
        {building.facilities && building.facilities.length > 0 && (
          <div className="mb-4">
            <h4 className="font-semibold text-gray-900 mb-2">Facilities</h4>
            <ul className="list-disc list-inside text-gray-700">
              {building.facilities.map((facility, index) => (
                <li key={index}>{facility}</li>
              ))}
            </ul>
          </div>
        )}
        
        {building.floors && (
          <div className="mb-4">
            <h4 className="font-semibold text-gray-900 mb-2">Number of Floors</h4>
            <p className="text-gray-700">{building.floors} floors</p>
          </div>
        )}
        
        <div className="flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default BuildingDetail;
