import { Component, OnInit, AfterViewInit } from '@angular/core';
import { ApiService } from '../services/api.service';
import * as L from 'leaflet';

@Component({
  selector: 'app-beosand',
  templateUrl: './beosand.component.html',
  styleUrls: ['./beosand.component.css']
})
export class BeosandComponent implements OnInit, AfterViewInit {
  private map: any;
  locations: any[] = [];
  upcomingEvents: any[] = [];
  filteredEvents: any[] = [];
  
  beachIcon = L.divIcon({
    className: 'beach-marker',
    iconSize: [40, 40]
  });

  constructor(private apiService: ApiService) { }

  ngOnInit(): void {
    this.apiService.getLocations().subscribe(data => {
      this.locations = data;
      this.addMarkersToMap();
    });

    this.apiService.getUpcomingEvents().subscribe(data => {
      this.upcomingEvents = data;
      this.filteredEvents = data;
    });
  }

  ngAfterViewInit(): void {
    this.initMap();
  }

  private initMap(): void {
    this.map = L.map('map', {
      center: [44.8125, 20.4612],
      zoom: 12
    });

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(this.map);
  }

  private addMarkersToMap(): void {
    this.locations.forEach(location => {
      const popupContent = `
        <h4>${location.name}</h4>
        <img src="${location.image_url || 'assets/default-location.png'}" alt="${location.name}" width="100">
        <br>
        <button class="btn-show-events">Show Events</button>
      `;
      
      const marker = L.marker([location.coordinates[0], location.coordinates[1]], { icon: this.beachIcon })
        .addTo(this.map)
        .bindPopup(popupContent);

      marker.on('popupopen', () => {
        const button = document.querySelector('.btn-show-events');
        button?.addEventListener('click', () => {
          this.apiService.getEventsByLocation(location.id).subscribe(data => {
            // TODO: Display events in a more user-friendly way
            alert(JSON.stringify(data));
          });
        });
      });
    });
  }
  
  filterEvents(eventType: string) {
    if (eventType === 'all') {
      this.filteredEvents = this.upcomingEvents;
    } else {
      this.filteredEvents = this.upcomingEvents.filter(event => event.event_type.toLowerCase() === eventType);
    }
  }

  joinEvent(eventId: string) {
    this.apiService.joinEvent(eventId).subscribe(response => {
      console.log('Joined event', response);
      // TODO: Add notification
    });
  }
} 