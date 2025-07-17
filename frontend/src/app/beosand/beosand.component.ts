import { Component, OnInit, AfterViewInit } from '@angular/core';
import { trigger, state, style, transition, animate } from '@angular/animations';
import { LocationService } from '../services/location.service';
import { Location } from '../models/location.model';
import * as L from 'leaflet';

@Component({
  selector: 'app-beosand',
  templateUrl: './beosand.component.html',
  styleUrls: ['./beosand.component.scss'],
  animations: [
    trigger('slideIn', [
      transition(':enter', [
        style({ transform: 'translateY(100%)', opacity: 0 }),
        animate('300ms ease-out', style({ transform: 'translateY(0)', opacity: 1 }))
      ]),
      transition(':leave', [
        animate('300ms ease-in', style({ transform: 'translateY(100%)', opacity: 0 }))
      ])
    ])
  ]
})
export class BeosandComponent implements OnInit, AfterViewInit {
  private map: any;
  private markers: L.Marker[] = [];
  private userLocationMarker?: L.Marker;
  
  locations: Location[] = [];
  filteredLocations: Location[] = [];
  selectedLocation: Location | null = null;
  searchTerm: string = '';
  mapStyle: 'street' | 'satellite' = 'street';
  userLocation: { lat: number; lng: number } | null = null;

  private beachIcon = L.icon({
    iconUrl: 'assets/beach-marker.png',
    iconSize: [38, 38],
    iconAnchor: [19, 38],
    popupAnchor: [0, -38]
  });

  private selectedIcon = L.icon({
    iconUrl: 'assets/beach-marker.png',
    iconSize: [48, 48],
    iconAnchor: [24, 48],
    popupAnchor: [0, -48],
    className: 'selected-marker'
  });

  private userIcon = L.icon({
    iconUrl: 'assets/user-marker.png',
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    className: 'user-marker'
  });

  constructor(private locationService: LocationService) { }

  ngOnInit(): void {
    this.loadLocations();
    this.getUserLocation();
  }

  ngAfterViewInit(): void {
    this.initMap();
  }

  private loadLocations(): void {
    this.locationService.getLocations().subscribe(locations => {
      this.locations = locations;
      this.filteredLocations = locations;
      this.calculateDistances();
      if (this.map) {
        this.addMarkersToMap();
      }
    });
  }

  private initMap(): void {
    this.map = L.map('map', {
      center: [44.8125, 20.4612], // Belgrade
      zoom: 12,
      zoomControl: false
    });

    // Add zoom control to bottom left
    L.control.zoom({
      position: 'bottomleft'
    }).addTo(this.map);

    this.updateMapStyle();
  }

  private updateMapStyle(): void {
    if (this.mapStyle === 'street') {
      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
      }).addTo(this.map);
    } else {
      L.tileLayer('https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}', {
        attribution: '&copy; Esri'
      }).addTo(this.map);
    }
  }

  private addMarkersToMap(): void {
    // Clear existing markers
    this.markers.forEach(marker => marker.remove());
    this.markers = [];

    // Add new markers
    this.filteredLocations.forEach(location => {
      if (location.coordinates) {
        const marker = L.marker(
          [location.coordinates.lat, location.coordinates.lon],
          { icon: this.beachIcon }
        );

        marker.on('click', () => {
          this.selectLocation(location);
        });

        marker.addTo(this.map);
        this.markers.push(marker);
      }
    });
  }

  private getUserLocation(): void {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          this.userLocation = {
            lat: position.coords.latitude,
            lng: position.coords.longitude
          };
          this.calculateDistances();
          this.showUserLocation();
        },
        (error) => {
          console.error('Error getting user location:', error);
        }
      );
    }
  }

  private showUserLocation(): void {
    if (this.userLocation && this.map) {
      if (this.userLocationMarker) {
        this.userLocationMarker.remove();
      }

      this.userLocationMarker = L.marker(
        [this.userLocation.lat, this.userLocation.lng],
        { icon: this.userIcon }
      ).addTo(this.map);
    }
  }

  private calculateDistances(): void {
    if (this.userLocation) {
      this.locations.forEach(location => {
        if (location.coordinates) {
          location.distance = this.calculateDistance(
            this.userLocation!.lat,
            this.userLocation!.lng,
            location.coordinates.lat,
            location.coordinates.lon
          );
        }
      });
      
      // Sort by distance
      this.filteredLocations.sort((a, b) => (a.distance || 0) - (b.distance || 0));
    }
  }

  private calculateDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371; // Radius of the Earth in kilometers
    const dLat = this.deg2rad(lat2 - lat1);
    const dLon = this.deg2rad(lon2 - lon1);
    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.deg2rad(lat1)) * Math.cos(this.deg2rad(lat2)) *
      Math.sin(dLon / 2) * Math.sin(dLon / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  private deg2rad(deg: number): number {
    return deg * (Math.PI / 180);
  }

  filterLocations(): void {
    if (!this.searchTerm) {
      this.filteredLocations = [...this.locations];
    } else {
      const term = this.searchTerm.toLowerCase();
      this.filteredLocations = this.locations.filter(location =>
        location.name.toLowerCase().includes(term) ||
        (location.address && location.address.toLowerCase().includes(term))
      );
    }
    this.addMarkersToMap();
  }

  selectLocation(location: Location): void {
    this.selectedLocation = location;
    
    // Center map on selected location
    if (location.coordinates) {
      this.map.setView([location.coordinates.lat, location.coordinates.lon], 15);
      
      // Update marker icon
      const markerIndex = this.filteredLocations.indexOf(location);
      if (markerIndex !== -1 && this.markers[markerIndex]) {
        this.markers.forEach((marker, index) => {
          marker.setIcon(index === markerIndex ? this.selectedIcon : this.beachIcon);
        });
      }
    }
  }

  closeLocationDetail(): void {
    this.selectedLocation = null;
    // Reset all markers to default icon
    this.markers.forEach(marker => marker.setIcon(this.beachIcon));
  }

  centerOnUserLocation(): void {
    if (this.userLocation) {
      this.map.setView([this.userLocation.lat, this.userLocation.lng], 14);
    } else {
      this.getUserLocation();
    }
  }

  toggleMapStyle(): void {
    this.mapStyle = this.mapStyle === 'street' ? 'satellite' : 'street';
    this.updateMapStyle();
  }

  viewLocationEvents(): void {
    if (this.selectedLocation) {
      // Navigate to events page with location filter
      console.log('View events for:', this.selectedLocation.name);
    }
  }

  getDirections(): void {
    if (this.selectedLocation && this.selectedLocation.coordinates) {
      const url = `https://www.google.com/maps/dir/?api=1&destination=${this.selectedLocation.coordinates.lat},${this.selectedLocation.coordinates.lon}`;
      window.open(url, '_blank');
    }
  }

  handleImageError(event: any): void {
    event.target.src = 'assets/default-location.png';
  }
} 