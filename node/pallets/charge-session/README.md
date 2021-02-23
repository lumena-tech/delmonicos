# charge-session pallet

A simple pallet which tracks charging sessions.

Two kind of accounts: user and charger

Workflow:
  - user requests a new charging session
  - charger confirms the request and records the beginning of the session
  - charger finishes the session and records the end of the session

