from google.oauth2 import service_account
from googleapiclient.discovery import build
import os
from pathlib import Path
import csv


def retrieve_and_save_sheets():
    # Set the path to the service account key file
    SERVICE_ACCOUNT_FILE = 'creds/sa-key.json'

    # Define the API scope (read-only access to Google Drive)
    SCOPES = [
        'https://www.googleapis.com/auth/drive.readonly',
        'https://www.googleapis.com/auth/spreadsheets.readonly'
    ]

    # Authenticate with the service account
    credentials = service_account.Credentials.from_service_account_file(
        SERVICE_ACCOUNT_FILE, scopes=SCOPES)


    # Initialize the Google Drive API
    drive_service = build('drive', 'v3', credentials=credentials)

    # Initialize the Google Sheets API
    sheets_service = build('sheets', 'v4', credentials=credentials)

    # Specify the search string (adjust to fit your needs)
    search_string = 'Monthly Budget'

    # Search for Google Spreadsheets in Drive
    query = f"name contains '{search_string}' and mimeType='application/vnd.google-apps.spreadsheet'"
    results = drive_service.files().list(q=query, fields="files(id, name)").execute()
    spreadsheets = results.get('files', [])

    if not spreadsheets:
        print('No spreadsheets found.')
    else:
        for spreadsheet in spreadsheets:
            spreadsheet_id = spreadsheet['id']
            spreadsheet_name = spreadsheet['name']

            print(f"Found Spreadsheet: {spreadsheet_name} (ID: {spreadsheet_id})")

            csv_dir = f"data/{spreadsheet_name}"

            if os.path.exists(csv_dir):
                print(f"'{spreadsheet_name}' data already exists.")
                continue

            # Get the list of sheets in the spreadsheet
            sheets_metadata = sheets_service.spreadsheets().get(spreadsheetId=spreadsheet_id).execute()
            sheets = sheets_metadata.get('sheets', [])

            # Download each sheet as CSV
            for sheet in sheets:
                sheet_title = sheet['properties']['title']

                result = sheets_service.spreadsheets().values().get(
                    spreadsheetId=spreadsheet_id,
                    range=sheet_title  # You can adjust the range if necessary
                ).execute()

                values = result.get('values', [])

                if not values:
                    print(f"No data found in sheet: {sheet_title}")
                else:
                    csv_filename = f"{csv_dir}/{sheet_title}.csv"
                    Path(csv_dir).mkdir(parents=True, exist_ok=True)

                    with open(csv_filename, 'w', newline='') as csvfile:
                        writer = csv.writer(csvfile)
                        writer.writerows(values)
                    print(f"Sheet '{sheet_title}' downloaded successfully as {csv_filename}")
                    print(f"Downloading sheet: {sheet_title} from {spreadsheet_name}")




if __name__ == "__main__":
    retrieve_and_save_sheets()
