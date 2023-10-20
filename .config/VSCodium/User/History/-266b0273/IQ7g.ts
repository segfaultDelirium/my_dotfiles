import { Component, Input, OnDestroy, Output } from '@angular/core';
import { Activity, CountryCode, Location, LocationToSend } from 'src/app/types';
import { LocationsService } from '../../locations.service';
import { Subscription } from 'rxjs';
import { MatDialog } from '@angular/material/dialog';
import { UpdateLocationDialogComponent } from 'src/app/dialogs/update-location-dialog/update-location-dialog.component';
import { ModificationResultDialogComponent } from 'src/app/dialogs/modification-result-dialog/modification-result-dialog.component';
import { TimelineDialogComponent } from 'src/app/dialogs/timeline-dialog/timeline-dialog.component';

@Component({
  selector: 'app-table',
  templateUrl: './table.component.html',
  styleUrls: ['./table.component.scss']
})
export class TableComponent implements OnDestroy{

  @Input() startDate: string | null = null;
  @Input() endDate: string | null = null;
  @Input() locations: Location[] = [];
  @Input() isReadOnly: boolean = false;

  displayedColumns: string[] = [
    "locationId", "isActive", "streetAddress", "city", 
  "zipcode", "state", "countryCode", "activityName", "companyName",
  "actions"
]
 
  updateSubscriptions: Subscription[] = [];
  deactivateSubscriptions: Subscription[] = [];
  reactivateSubscriptions: Subscription[] = [];

  constructor(public dialog: MatDialog, private locationsService: LocationsService){}

  ngOnDestroy(): void {
    this.updateSubscriptions.forEach(subscription => subscription.unsubscribe());
    this.deactivateSubscriptions.forEach(subscription => subscription.unsubscribe());
    this.reactivateSubscriptions.forEach(subscription => subscription.unsubscribe());
  }

  printLocations(){
    console.log(this.locations);
  }

  deactivate(locationId: number){
    const subscription = this.locationsService.deactivateLocation(locationId).subscribe(result => {
      this.dialog.open(ModificationResultDialogComponent, {
        data: result
      });
    })
    this.deactivateSubscriptions.push(subscription);

  }

  reactivate(locationId: number){
    const subscription = this.locationsService.reactivateLocation(locationId).subscribe(result => {
      this.dialog.open(ModificationResultDialogComponent, {
        data: result
      });
    })
    this.reactivateSubscriptions.push(subscription);
  }

  openEditDialog(location: Location){
    // get location from server with location.id

    const currentlySavedLocation: Readonly<Location> = JSON.parse(JSON.stringify(location));
    const dialogRef = this.dialog.open(UpdateLocationDialogComponent, {
      data: {
        location: JSON.parse(JSON.stringify(location)),
        currentlySavedLocation: currentlySavedLocation,
      },
    });

    dialogRef.afterClosed().subscribe(result => {
      if(result === undefined) return;
      if(result === '') return;

    });
  }

  openTimelineDialog(locationId: number){
    const dialogRef = this.dialog.open(TimelineDialogComponent, {
      data: {
        locationId: locationId,
        startDate: this.startDate,
        endDate: this.endDate
      },
    });

    dialogRef.afterClosed().subscribe(result => {
      if(result === undefined) return;
      if(result === '') return;

    });
  }

}
