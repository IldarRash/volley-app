import { Component, OnInit } from '@angular/core';
import { ApiService } from '../services/api.service';
import { Subscription } from '../models/subscription.model';
import { AuthService } from '../auth/auth.service';

@Component({
  selector: 'app-subscriptions',
  templateUrl: './subscriptions.component.html',
  styleUrls: ['./subscriptions.component.css']
})
export class SubscriptionsComponent implements OnInit {
  subscriptions: Subscription[] = [];

  constructor(
    private apiService: ApiService,
    private authService: AuthService
  ) { }

  ngOnInit(): void {
    this.apiService.getSubscriptions().subscribe(
      (res) => {
        this.subscriptions = res;
      },
      (err) => {
        console.error('Error getting subscriptions', err);
      }
    );
  }

  purchase(subscriptionId: string): void {
    const userId = this.authService.getUserId();
    if (userId) {
      this.apiService.purchaseSubscription(userId, subscriptionId).subscribe(
        (res) => {
          console.log('Subscription purchased successfully', res);
        },
        (err) => {
          console.error('Error purchasing subscription', err);
        }
      );
    }
  }
}
