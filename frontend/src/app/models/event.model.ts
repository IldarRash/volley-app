export interface Event {
  id: string;
  event_type: string;
  datetime: Date;
  location_id: string;
  participants: string[];
  max_participants: number;
  confirmed: boolean;
} 