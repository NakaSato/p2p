export interface BuildingInfo {
  id: number;
  name: string;
  nameEn: string;
  type: 'academic' | 'library' | 'facility' | 'utcc' | 'admin';
  x: number;
  y: number;
  width: number;
  height: number;
  description?: string;
  facilities?: string[];
  floors?: number;
}

export interface CampusMapProps {
  onBuildingSelect?: (building: BuildingInfo) => void;
  selectedBuildingId?: number;
  className?: string;
}

export interface BuildingDetailProps {
  building: BuildingInfo | null;
  onClose: () => void;
}
