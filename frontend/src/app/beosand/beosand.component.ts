import { Component, OnInit } from '@angular/core';
import { EventService } from '../services/event.service';
import { Event } from '../models/event.model';
import { Observable } from 'rxjs';

@Component({
  selector: 'app-beosand',
  templateUrl: './beosand.component.html',
  styleUrls: ['./beosand.component.scss']
})
export class BeosandComponent implements OnInit {
  events$: Observable<Event[]>;

  constructor(private eventService: EventService) { }

  ngOnInit(): void {
    this.events$ = this.eventService.getEventsStream();
    this.eventService.getEvents().subscribe();
  }
} 