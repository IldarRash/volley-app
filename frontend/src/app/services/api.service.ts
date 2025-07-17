import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthService } from '../auth/auth.service';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  private apiUrl = 'http://localhost:8080';

  constructor(private http: HttpClient, private authService: AuthService) { }

  private getHeaders(): HttpHeaders {
    const token = this.authService.getToken();
    return new HttpHeaders().set('Authorization', `Bearer ${token}`);
  }

  get(endpoint: string): Observable<any> {
    return this.http.get(`${this.apiUrl}${endpoint}`, { headers: this.getHeaders() });
  }

  post(endpoint: string, data: any): Observable<any> {
    return this.http.post(`${this.apiUrl}${endpoint}`, data, { headers: this.getHeaders() });
  }

  put(endpoint: string, data: any): Observable<any> {
    return this.http.put(`${this.apiUrl}${endpoint}`, data, { headers: this.getHeaders() });
  }

  delete(endpoint: string): Observable<any> {
    return this.http.delete(`${this.apiUrl}${endpoint}`, { headers: this.getHeaders() });
  }

  // Old methods to be kept for now
  getLocations(): Observable<any[]> {
    return this.get('/locations');
  }

  getUpcomingEvents(): Observable<any[]> {
    return this.get('/events/upcoming');
  }

  getEventsByLocation(locationId: string): Observable<any[]> {
    return this.get(`/locations/${locationId}/events`);
  }

  joinEvent(eventId: string): Observable<any> {
    return this.post(`/events/${eventId}/join`, {});
  }

  // Admin endpoints
  createLocation(location: any): Observable<any> {
    return this.post('/admin/locations', location);
  }

  confirmLocation(locationId: string): Observable<any> {
    return this.put(`/admin/locations/${locationId}/confirm`, {});
  }

  createEvent(event: any): Observable<any> {
    return this.post('/admin/events', event);
  }

  confirmEvent(eventId: string): Observable<any> {
    return this.put(`/admin/events/${eventId}/confirm`, {});
  }
} 