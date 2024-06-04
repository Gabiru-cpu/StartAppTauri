import { Routes } from "@angular/router";
import { LoginComponent } from "./presentation/pages/public-routing/login/login.component";
import { HomeComponent } from "./presentation/pages/dash-routing/home/home.component";

export const routes: Routes = [
    {path: '', component: LoginComponent},
    {path: 'home', component:HomeComponent},
    //{path: 'home', component:HomeComponent, canActivate: [AuthGuard]},

    { path: '**', redirectTo: '' }
];
