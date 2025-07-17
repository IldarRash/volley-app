import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ApiService } from '../../services/api.service';
import { Location } from '../../models/location.model';

@Component({
  selector: 'app-add-location',
  templateUrl: './add-location.component.html',
  styleUrls: ['./add-location.component.css']
})
export class AddLocationComponent {
  locationForm: FormGroup;

  constructor(
    private fb: FormBuilder,
    private apiService: ApiService
  ) {
    this.locationForm = this.fb.group({
      name: ['', Validators.required],
      address: ['', Validators.required],
      latitude: [0, Validators.required],
      longitude: [0, Validators.required],
      description: [''],
      type: ['classic', Validators.required]
    });
  }

  onSubmit(): void {
    if (this.locationForm.valid) {
      const newLocation: Location = this.locationForm.value;
      newLocation.confirmed = false; // By default, new locations are not confirmed
      this.apiService.addLocation(newLocation).subscribe(
        (res) => {
          console.log('Location added successfully', res);
          this.locationForm.reset();
        },
        (err) => {
          console.error('Error adding location', err);
        }
      );
    }
  }
}
