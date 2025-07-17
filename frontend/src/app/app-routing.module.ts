import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { BeosandComponent } from './beosand/beosand.component';
import { LoginComponent } from './auth/login/login.component';
import { RegisterComponent } from './auth/register/register.component';
import { AdminComponent } from './admin/admin.component';
import { ProfileComponent } from './profile/profile.component';
import { AuthGuard } from './auth/auth.guard';
import { ClassicComponent } from './classic/classic.component';
import { CampsComponent } from './camps/camps.component';
import { SubscriptionsComponent } from './subscriptions/subscriptions.component';

const routes: Routes = [
  { path: '', redirectTo: '/classic', pathMatch: 'full' },
  { path: 'classic', component: ClassicComponent },
  { path: 'beach', component: BeosandComponent },
  { path: 'camps', component: CampsComponent },
  { path: 'login', component: LoginComponent },
  { path: 'register', component: RegisterComponent },
  { path: 'admin', component: AdminComponent, canActivate: [AuthGuard] },
  { path: 'profile', component: ProfileComponent, canActivate: [AuthGuard] },
  { path: 'subscriptions', component: SubscriptionsComponent, canActivate: [AuthGuard] },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { } 