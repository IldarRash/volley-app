import { Component, OnInit } from '@angular/core';
import { ApiService } from '../services/api.service';
import { User } from '../models/user.model';
import { AuthService } from '../auth/auth.service';

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html',
  styleUrls: ['./profile.component.css']
})
export class ProfileComponent implements OnInit {
  user: User | null = null;

  constructor(
    private apiService: ApiService,
    private authService: AuthService
  ) { }

  ngOnInit(): void {
    const userId = this.authService.getUserId();
    if (userId) {
      this.apiService.getUser(userId).subscribe(
        (res) => {
          this.user = res;
        },
        (err) => {
          console.error('Error getting user', err);
        }
      );
    }
  }

  updateProfile(): void {
    if (this.user) {
      this.apiService.updateUser(this.user.id, this.user).subscribe(
        (res) => {
          console.log('Profile updated successfully', res);
        },
        (err) => {
          console.error('Error updating profile', err);
        }
      );
    }
  }
}
