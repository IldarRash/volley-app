export interface Event {
  _id: any;
  type: 'training' | 'game' | 'tournament';
  location_id: any;
  datetime: string;
  level: 'beginner' | 'intermediate' | 'advanced';
  price: number;
  trainer_id?: any;
  confirmed: boolean;
  participants: {
    user_id: any;
    status: 'pending' | 'confirmed';
    payment: 'manual' | 'pass' | 'unpaid';
  }[];
} 