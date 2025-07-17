import { Component, OnInit, AfterViewInit } from '@angular/core';
import { LocationService } from '../services/location.service';
import { Location } from '../models/location.model';
import * as L from 'leaflet';

@Component({
  selector: 'app-beosand',
  templateUrl: './beosand.component.html',
  styleUrls: ['./beosand.component.scss']
})
export class BeosandComponent implements OnInit, AfterViewInit {
  private map: any;
  locations: Location[] = [];

  private beachIcon = L.icon({
    iconUrl: 'assets/beach-marker.png',
    iconSize: [38, 38],
    iconAnchor: [19, 38],
    popupAnchor: [0, -38]
  });

  constructor(private locationService: LocationService) { }

  ngOnInit(): void {
    this.locationService.getLocations().subscribe(locations => {
      this.locations = locations.filter(loc => loc.type === 'beach');
      if (this.map) {
        this.addMarkersToMap();
      }
    });
  }

  ngAfterViewInit(): void {
    this.initMap();
    if (this.locations.length > 0) {
      this.addMarkersToMap();
    }
  }

  private initMap(): void {
    this.map = L.map('map', {
      center: [44.8125, 20.4612], // Belgrade
      zoom: 12
    });

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(this.map);
  }

  private addMarkersToMap(): void {
    this.locations.forEach(location => {
      if(location.coordinates) {
        const popupContent = `
          <b>${location.name}</b><br>${location.address}
        `;
        L.marker([location.coordinates[0], location.coordinates[1]], { icon: this.beachIcon })
          .addTo(this.map)
          .bindPopup(popupContent);
      }
    });
  }
} 