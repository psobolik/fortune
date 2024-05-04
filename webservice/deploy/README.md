# Deploy Fortune API
To deploy the Fortune API on `marconi`, an Apache server.

1. Copy the data files to the server.
   * Since this app's data files are compatible with the BSD program (`fortune-mod`), we put them where that program would, i.e. `/usr/share/games/fortunes`. If we ever install `fortune-mod` on `marconi`, they will both use all the files in that folder.
1. Install the application files in `/srv/fortuneapi`

   | File          | Description                                                                                                                                                                                 |
   |---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
   | `fortuneapi`  | The executable                                                                                                                                                                              |
   | `Config.toml` | This tells the program where to find the data files. <br/>There's only one `Config.toml` file, so it *must be edited* to set the correct path on `marconi`, i.e. `/usr/share/games/fortune` |
   
1. Configure `systemd` to run the executable as a service.
   * Copy `fortuneapi.service` to `/etc/systemd/system`
   * Enable the service: `$ sudo systemctl enable fortuneapi`
1. Configure Apache2 to serve the API as a reverse proxy.
   * Copy `fortuneapi.conf` to `/etc/apache2/sites-available`. 
   * Enable the reverse proxy: `sudo a2ensite fortuneapi.conf`[^5]

## Notes:
When it is built for release, Rocket uses the `release` port in `Rocket.toml`, i.e. 4040. 

If all goes will, Apache will serve the API at http://marconi/api/fortune.
