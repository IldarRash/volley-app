export interface User {
  id: string;
  username: string;
  telegram_id?: string;
  instagram_link?: string;
  image_url?: string;
  rating: number;
  role: string;
  subscribed: boolean;
  subscriptions: any[];
} 