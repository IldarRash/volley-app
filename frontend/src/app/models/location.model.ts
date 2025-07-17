export interface Coordinates {
  lat: number;
  lon: number;
}

export interface Location {
  id: string;
  name: string;
  address?: string;
  coordinates: Coordinates;
  confirmed: boolean;
  image_url?: string;
  distance?: number;
  events_count?: number;
  rating?: number;
  phone?: string;
  website?: string;
} 