import { Component, OnInit } from '@angular/core';
import { LocationService } from '../services/location.service';
import { EventService } from '../services/event.service';
import { Location } from '../models/location.model';
import { Event } from '../models/event.model';
import * as L from 'leaflet';

@Component({
  selector: 'app-beosand',
  templateUrl: './beosand.component.html',
  styleUrls: ['./beosand.component.scss']
})
export class BeosandComponent implements OnInit {
  locations: Location[] = [];
  events: Event[] = [];
  selectedLocation: Location | null = null;
  private map: any;

  constructor(
    private locationService: LocationService,
    private eventService: EventService
  ) {}

  ngOnInit(): void {
    this.locationService.getLocations().subscribe(
      (data) => {
        this.locations = data;
        this.initMap();
        this.addMarkers();
      },
      (error) => {
        console.error('Error fetching locations', error);
      }
    );
  }

  private initMap(): void {
    this.map = L.map('map', {
      center: [44.7866, 20.4489], // Belgrade coordinates
      zoom: 12
    });

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(this.map);
  }

  private addMarkers(): void {
    this.locations.forEach(location => {
      if (location.coordinates) {
        const marker = L.marker(location.coordinates as L.LatLngTuple).addTo(this.map);
        marker.bindPopup(`<b>${location.name}</b>`);
        marker.on('click', () => {
          this.onLocationSelect(location);
        });
      }
    });
  }

  onLocationSelect(location: Location): void {
    this.selectedLocation = location;
    // The location._id object from MongoDB is structured as { $oid: "..." }, so we access the actual ID string.
    this.eventService.getEventsByLocation(location._id.$oid).subscribe(
      (data) => {
        this.events = data;
      },
      (error) => {
        console.error(`Error fetching events for location ${location.name}`, error);
        this.events = [];
      }
    );
  }
} 