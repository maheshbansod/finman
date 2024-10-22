from google.oauth2 import service_account
from googleapiclient.discovery import build
import io
from googleapiclient.http import MediaIoBaseDownload


def main():
    print("Hello from sheet-analysis!")

    # Set the path to the service account key file
    SERVICE_ACCOUNT_FILE = 'creds/sa-key.json'

    # Define the API scope (read-only access to Google Drive)
    SCOPES = ['https://www.googleapis.com/auth/drive.readonly']

    # Authenticate with the service account
    credentials = service_account.Credentials.from_service_account_file(
        SERVICE_ACCOUNT_FILE, scopes=SCOPES)

    # Build the Google Drive service
    service = build('drive', 'v3', credentials=credentials)

    # Example: List files in the Drive (optional, to verify it works)
    results = service.files().list(
        pageSize=10, fields="nextPageToken, files(id, name)").execute()
    items = results.get('files', [])

    if not items:
        print('No files found.')
    else:
        print('Files:')
        for item in items:
            print(f"{item['name']} (ID: {item['id']})")


if __name__ == "__main__":
    main()
