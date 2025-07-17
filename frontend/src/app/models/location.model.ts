export interface Location {
  id: string;
  name: string;
  address: string;
  coordinates: [number, number];
  type: 'classic' | 'beach';
  confirmed: boolean;
  image_url?: string;
} 