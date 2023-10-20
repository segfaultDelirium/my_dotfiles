import { Injectable } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { catchError, share } from 'rxjs';
import { environment } from 'src/environments/environment';
import { Environments } from 'src/environments/environmentsEnum';


let apiBaseUrl = "http://localhost:8080/";

switch(environment.env){
  case Environments.LOCAL: { 
    apiBaseUrl = Environments.LOCAL;
    break; 
  }
  case Environments.REMOTE_PROD: { 
    apiBaseUrl = Environments.REMOTE_PROD; 
    break; 
  }
}

console.log(`Backend api: ${apiBaseUrl}`)
export enum ApiEndpoint{
  ACTIVITIES = 'activities',
  COUNTRY_CODES = 'country_codes',
  LOCATIONS = 'locations',
  LOCATIONS_AT_GIVEN_TIME = 'locations/at-given-time', // example /locations-at-given-time/2023-04-13 23:45:16
  LOCATIONS_TIMELINE = 'locations/timeline',
  LOCATIONS_CHANGES = 'locations/changes', // example /locations/changes?startDate=2034-11-23&endDate=1999-05-12
  
}

export enum ApiMethod {
  'GET',
  'POST',
  'PUT',
  'DELETE'
}


@Injectable({
  providedIn: 'root'
})
export class HttpService {

  constructor(private http: HttpClient) { }

  requestCall(apiMethod: ApiMethod, apiEndpoint: ApiEndpoint | string, data?: any){
    const url = `${apiBaseUrl}${apiEndpoint}`;
    let response;
      switch(apiMethod){
        case ApiMethod.GET:
          response = this.http.get(url);
          break;
        case ApiMethod.POST:
          response = this.http.post(url, data);
          break;
        case ApiMethod.PUT:
          response = this.http.put(url, data);
          break;
        case ApiMethod.DELETE:
          response = this.http.delete(url);
          break;
      }
      return response?.pipe(share())
      .pipe(catchError( async err => this.handleError(err)));
  }
 
  private handleError(error: HttpErrorResponse){
    console.error(JSON.stringify(error));
    // if(error instanceof HttpErrorResponse){
    //   this._error.whichError(error.status, error.message);
    //   throw error;
    // }
    // console.error('An unknown error occured:', error);

  }

  public printEnvironment(){
    console.log(environment);
    console.log(apiBaseUrl);
  }
}
