import axios from "axios";

const API_URL = "http://127.0.0.1:8080";

export const fetchGallery = async () => {
    try {
        const response = await axios.get(`${API_URL}/gallery`);
        return response.data;
    } catch (error) {
        console.error("Erreur lors de la récupération de la galerie :", error);
        throw error;
    }
};

export const addGalleryItem = async (item) => {
    try {
        const response = await axios.post(`${API_URL}/gallery`, item);
        return response.data;
    } catch (error) {
        console.error("Erreur lors de l'ajout d'un élément à la galerie :", error);
        throw error;
    }
};
