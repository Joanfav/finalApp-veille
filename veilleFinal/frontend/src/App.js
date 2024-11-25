import React, { useEffect, useState } from "react";
import { fetchGallery, addGalleryItem } from "./providers/providerImage";
import "./App.css";

function App() {
    const [gallery, setGallery] = useState([]);
    const [error, setError] = useState(null);
    const [selectedImage, setSelectedImage] = useState(null);

    const handleImageChange = (event) => {
        const file = event.target.files[0];
        if (file) {
            const imageUrl = URL.createObjectURL(file);
            setSelectedImage(imageUrl);
        }
    };

    useEffect(() => {
        const fetchData = async () => {
            try {
                const data = await fetchGallery();
                setGallery(data);
            } catch (err) {
                console.error("Error fetching gallery:", err);
                setError("Impossible de charger la galerie.");
            }
        };

        fetchData();
    }, []);

    const handleAddItem = async () => {
        const newItem = { name: "New Item", image_path: "/path/to/image" };
        try {
            const addedItem = await addGalleryItem(newItem);
            setGallery((prevGallery) => [...prevGallery, addedItem]);
        } catch (err) {
            console.error("Error adding gallery item:", err);
            setError("Impossible d'ajouter un élément à la galerie.");
        }
    };

    return (
        <div>
            <div className="background">
                <div className="containerImagePrincipal">
                    <div className="image-preview">
                        {selectedImage ? (
                            <img src={selectedImage} alt="Preview" className="preview-image"/>
                        ) : (
                            <p>Aucune image sélectionnée</p>
                        )}
                    </div>
                    <input
                        type="file"
                        accept="image/*"
                        onChange={handleImageChange}
                        className="upload-button"
                    />
                    <div className="sliders">
                        <div className="slider">
                            <label>Rotation</label>
                            <input type="range" min="0" max="360"/>
                        </div>
                        <div className="slider">
                            <label>Luminosité</label>
                            <input type="range" min="0" max="100"/>
                        </div>
                        <div className="slider">
                            <label>Redimensionnement</label>
                            <input type="range" min="50" max="200"/>
                        </div>
                    </div>
                </div>
            </div>
            <div className="backgroundHistorique">
                <div className="gallery-container">
                    {gallery.map((item) => (
                        <div
                            key={item.id || item.name}
                            className="gallery-card"
                            onClick={() => alert(`Vous avez cliqué sur ${item.name}`)}
                        >
                            <img src={item.image_path} alt={item.name} className="gallery-image"/>
                            <div className="gallery-info">
                                <strong>{item.name}</strong>
                            </div>
                        </div>
                    ))}
                </div>
            </div>


        </div>
    );
}

export default App;
