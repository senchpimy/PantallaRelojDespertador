import spotipy
from spotipy.oauth2 import SpotifyOAuth

class SpotifyController:
    def __init__(self, 
                 client_id="",
                 client_secret="",
                 redirect_uri="http://localhost:8888/callback",
                 scope="user-modify-playback-state user-read-playback-state"):
        """
        Inicializa el controlador de Spotify.
        Args:
            client_id (str): ID de cliente de la aplicación Spotify
            client_secret (str): Secret de cliente de la aplicación
            redirect_uri (str): URI de redirección para autenticación
            scope (str): Permisos de la API requeridos
        """
        self.sp = spotipy.Spotify(auth_manager=SpotifyOAuth(
            client_id=client_id,
            client_secret=client_secret,
            redirect_uri=redirect_uri,
            scope=scope
        ))
        self.device_id = self._find_active_device()
        
    def _find_active_device(self):
        """Busca y retorna el ID del dispositivo activo"""
        devices = self.sp.devices()
        for device in devices.get('devices', []):
            if device.get('is_active', False):
                print(f"Dispositivo activo encontrado: {device['name']}")
                return device['id']
        print("No se encontró dispositivo activo")
        return None

    def play(self, track_uri=None):
        """Inicia/reanuda la reproducción"""
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            if track_uri:
                self.sp.start_playback(device_id=self.device_id, uris=[track_uri])
            else:
                self.sp.start_playback(device_id=self.device_id)
            print("Reproducción iniciada")
        except spotipy.SpotifyException as e:
            print(f"Error al reproducir: {e}")

    def pause(self):
        """Pausa la reproducción"""
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.pause_playback(device_id=self.device_id)
            print("Reproducción pausada")
        except spotipy.SpotifyException as e:
            print(f"Error al pausar: {e}")

    def state(self):
        """Obtiene el estado actual de reproducción"""
        playback = self.sp.current_playback()
        if not playback:
            return "No hay reproducción activa"
        return "Reproduciendo" if playback['is_playing'] else "Pausado"

    def artist_name(self):
        """Obtiene el/los nombre(s) del artista actual"""
        playback = self.sp.current_playback()
        if playback and playback.get('item'):
            return ', '.join([artist['name'] for artist in playback['item']['artists']])
        return "No disponible"

    def album_cover(self, size=0):
        """Obtiene la URL de la portada del álbum actual
        Args:
            size (int): Índice del tamaño de imagen deseado (0 = mayor tamaño)
        """
        playback = self.sp.current_playback()
        if playback and playback.get('item'):
            images = playback['item']['album'].get('images', [])
            if images:
                return images[size]['url'] if size < len(images) else images[0]['url']
        return None

    def update_device(self):
        """Actualiza el dispositivo activo"""
        self.device_id = self._find_active_device()
        return self.device_id is not None

    def next(self):
        """Pasa a la siguiente pista en la cola de reproducción."""
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.next_track(device_id=self.device_id)
            print("Siguiente pista")
        except spotipy.SpotifyException as e:
            print(f"Error al pasar a la siguiente pista: {e}")
    
    def prev(self):
        """Vuelve a la pista anterior en la cola de reproducción."""
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.previous_track(device_id=self.device_id)
            print("Pista anterior")
        except spotipy.SpotifyException as e:
            print(f"Error al volver a la pista anterior: {e}")

    def current_track_name(self):
        """Obtiene el nombre de la pista que se está reproduciendo."""
        playback = self.sp.current_playback()
        if playback and playback.get('item'):
            return playback['item']['name']
        return "No disponible"

    def playback_progress(self):
        """Obtiene el progreso actual de la reproducción en milisegundos."""
        playback = self.sp.current_playback()
        if playback:
            return playback.get('progress_ms', 0)
        return 0

    def current_track_duration(self):
        """Obtiene la duración total de la pista actual en milisegundos."""
        playback = self.sp.current_playback()
        if playback and playback.get('item'):
            return playback['item']['duration_ms']
        return 0

    def seek(self, position_ms):
        """Salta a una posición específica en la pista actual.
        Args:
            position_ms (int): Posición en milisegundos
        """
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.seek_track(position_ms, device_id=self.device_id)
            print(f"Saltando a la posición {position_ms} ms")
        except spotipy.SpotifyException as e:
            print(f"Error al saltar: {e}")

    def set_repeat(self, state):
        """Establece el modo de repetición.
        Args:
            state (str): "track", "context", o "off"
        """
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.repeat(state, device_id=self.device_id)
            print(f"Modo de repetición establecido a: {state}")
        except spotipy.SpotifyException as e:
            print(f"Error al establecer el modo de repetición: {e}")

    def set_shuffle(self, state):
        """Activa o desactiva el modo shuffle.
        Args:
            state (bool): True para activar, False para desactivar
        """
        if not self.device_id:
            print("Error: Ningún dispositivo disponible")
            return
        try:
            self.sp.shuffle(state, device_id=self.device_id)
            print(f"Modo shuffle {'activado' if state else 'desactivado'}")
        except spotipy.SpotifyException as e:
            print(f"Error al establecer el modo shuffle: {e}")

    def get_shuffle_state(self):
        """Obtiene el estado actual del modo shuffle."""
        playback = self.sp.current_playback()
        return playback.get('shuffle_state', False) if playback else False
    
    def is_playing(self):
        """Retorna True si la reproducción está activa."""
        playback = self.sp.current_playback()
        return playback.get('is_playing', False) if playback else False
