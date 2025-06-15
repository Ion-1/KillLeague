# KillLeague
An application to kill the League of Legends process. That's it.

For when opening Task Manager and searching for the process is too tedious.

Has four buttons:
1. Kill the game
2. Kill the League client
3. Kill the Riot client
4. Kill the Riot service (breaks the League client if it is still open)

## Installation
> [!NOTE]
> Windows only, since I can't test on Mac.

Download the executable from Releases.

To have Windows recognize the application:
1. Right-click the file in Windows Explorer and press `Create shortcut`.
2. Rename the shortcut whatever you like (this will be the name Windows recognizes)
3. Press Windows + R, which opens the Run Dialog, and type:
   - `shell:programs` for local user (may require a restart)
   - `shell:common programs` for all users (requires admin permissions for the next step)
4. Press Enter. Drag the shortcut you created into the folder that pops up.
5. Done!

## Building
Install Qt 6 and make sure it is in path, then build the application with cargo.
