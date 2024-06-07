## WBS Announcement REST API

### Overview
In partnership with the Wolmer's Boys' School, this REST API is intended to allow club presidents, teachers and other executive members of the school body to effortlessly share announcements to the administrative prefect assigned for that school devotion. 

As an ex-admin prefect myself, those preceding minutes in getting ready to share announcements is as hectic as it can be. Various members of the school community are frantically approaching you left-and-right to share their individual notices, and it's a really draining experience (especially for those who aren't as social as myself). 

By virtue of that, this project is intended to provide real-time data on announcements around the school.

### URI
So as for now this isn't a deployed API, thus the first iteration of the URI's will be prefixed with the banal `https://localhost:8080` and is structured as described below. Also take into consideration that a self-signed certificate was used.

### Announcements
Announcements can be retrieved based on how you intend on filtering the data. Either way, the claimed JSON data includes the announcement UUID, the announcement itself, the date on which the it should be/was announced and the UUID of the club that's sharing the information. 

For example, getting all announcements: `https://localhost:8080/announcement`
```json
[
    {
        "announcement_uid": "35070048-f081-4fda-bf20-2f77819c4c93",
        "info": "For those within the WBS Coding Club that attended the recent interclubbing with Immaculate, you are asked to please stay back after devotion",
        "date": "2024-03-18",
        "club_uid": "05b4e952-0410-4745-9ebd-3396d8c47da8"
    },
    {
        "announcement_uid": "3144bcb3-cf74-4a1e-b8b7-853051431d4d",
        "info": "The Entrepreneurship Club will be having a cake sale this week Wednesday. Please come out and give your support.",
        "date": "2024-03-18",
        "club_uid": "24007576-ee06-44e9-8763-b610b28ecb4a"
    },
    {
        "announcement_uid": "913212cc-bcc1-4605-b2d1-e36dee3f5298",
        "info": "The Entreprenuership Club would like to thank everyone that supported the cake sale. However, for the ones who haven't payed for their cake, you are asked to do so by tomorrow",
        "date": "2024-03-25",
        "club_uid": "24007576-ee06-44e9-8763-b610b28ecb4a"
    }
]
```
getting announcements by club UUID: `https://localhost:8080/announcement/club/05b4e952-0410-4745-9ebd-3396d8c47da8`
```json
[
    {
        "announcement_uid": "35070048-f081-4fda-bf20-2f77819c4c93",
        "info": "For those within the WBS Coding Club that attended the recent interclubbing with Immaculate, you are asked to please stay back after devotion",
        "date": "2024-03-18",
        "club_uid": "05b4e952-0410-4745-9ebd-3396d8c47da8"
    }
]
```
getting announcements by club UUID and date: `https://localhost:8080/announcement/club/24007576-ee06-44e9-8763-b610b28ecb4a/2024-03-25`
```json
[
  {
    "announcement_uid": "913212cc-bcc1-4605-b2d1-e36dee3f5298",
    "info": "The Entreprenuership Club would like to thank everyone that supported the cake sale. However, for the ones who haven't payed for their cake, you are asked to do so by tomorrow",
    "date": "2024-03-25",
    "club_uid": "24007576-ee06-44e9-8763-b610b28ecb4a"
  }
]
```
getting announcements by date: `https://localhost:8080/announcement/date/2024-03-18`
```json
[
    {
        "announcement_uid": "35070048-f081-4fda-bf20-2f77819c4c93",
        "info": "For those within the WBS Coding Club that attended the recent interclubbing with Immaculate, you are asked to please stay back after devotion",
        "date": "2024-03-18",
        "club_uid": "05b4e952-0410-4745-9ebd-3396d8c47da8"
    },
    {
        "announcement_uid": "3144bcb3-cf74-4a1e-b8b7-853051431d4d",
        "info": "The Entrepreneurship Club will be having a cake sale this week Wednesday. Please come out and give your support.",
        "date": "2024-03-18",
        "club_uid": "24007576-ee06-44e9-8763-b610b28ecb4a"
    }
]
```
