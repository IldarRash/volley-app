import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, Observable } from 'rxjs';
import { Event } from '../models/event.model';
import { WebsocketService } from './websocket.service';
import { tap } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class EventService {
  private apiUrl = 'http://localhost:8000';
  private events$ = new BehaviorSubject<Event[]>([]);

  constructor(
    private http: HttpClient,
    private websocketService: WebsocketService
  ) {
    this.websocketService.getMessage().subscribe((message: any) => {
      if (message.type === 'event_updated') {
        const event: Event = message.payload;
        const currentEvents = this.events$.getValue();
        const index = currentEvents.findIndex(e => e.id === event.id);
        if (index > -1) {
          currentEvents[index] = event;
          this.events$.next([...currentEvents]);
        }
      }
    });
  }

  getEvents(): Observable<Event[]> {
    return this.http.get<Event[]>(`${this.apiUrl}/events`).pipe(
      tap(events => this.events$.next(events))
    );
  }

  getEventsStream(): Observable<Event[]> {
    return this.events$.asObservable();
  }
} 