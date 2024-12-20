import React, {useEffect, useState} from "react";
import axios from "axios";
import "./App.css";
import ImageDetails from "./pages/ImageDetails";

function App() {
    const [gallery, setGallery] = useState([]);
    const [error, setError] = useState(null);
    const [selectedImage, setSelectedImage] = useState(null);
    const [name, setName] = useState("");
    const [rotation, setRotation] = useState(0);
    const [brightness, setBrightness] = useState(100);
    const [cropX, setCropX] = useState(0);
    const [cropY, setCropY] = useState(0);
    const [message, setMessage] = useState("");
    const [fileName, setFileName] = useState("");
    const [activeRotation, setActiveRotation] = useState(null);
    const [selectedImageId, setSelectedImageId] = useState(null);
    const [groupedImages, setGroupedImages] = useState({});

    const handleImageClick = (id) => {
        setSelectedImageId(id);
    };

    const closeImageDetails = () => {
        setSelectedImageId(null);
    };


    const handleSubmit = async (e) => {
        e.preventDefault();

        if (!selectedImage) {
            setMessage("Please select an image.");
            return;
        }

        const formData = new FormData();
        formData.append("name", name);
        formData.append("rotation", rotation);
        formData.append("brightness", brightness);
        formData.append("crop_x", cropX);
        formData.append("crop_y", cropY);
        formData.append("file", selectedImage);

        try {
            const response = await fetch("http://localhost:8080/upload", {
                method: "POST",
                body: formData,
            });

            if (response.ok) {
                setMessage("Image uploaded successfully");
                fetchGallerys();
                setRotation(0);
                setBrightness(0);
                setActiveRotation(null);
            } else {
                const text = await response.text();
                console.error("Error response from server:", text);
                setMessage("Error uploading image");
            }
        } catch (error) {
            console.error("Error during fetch:", error);
            setMessage("Error uploading image");
        }
    };

    const handleTest = async (e) => {
        e.preventDefault();

        if (!selectedImage) {
            setMessage("Please select an image.");
            return;
        }
        const formData = new FormData();
        formData.append("name", name);
        formData.append("rotation", rotation);
        formData.append("brightness", brightness);
        formData.append("crop_x", cropX);
        formData.append("crop_y", cropY);
        formData.append("file", selectedImage);

        try {
            const response = await fetch("http://localhost:8080/testImage", {
                method: "POST",
                body: formData,
            });

            if (response.ok) {
                const imageData = await response.text();
                setSelectedImage(imageData);
                setRotation(0);
                setBrightness(0);
                setActiveRotation(null);
            }
        } catch (error) {
            console.error("Error during fetch:", error);
            setMessage("Error applying image transformation.");
        }
    };

    useEffect(() => {
        fetchGallerys();
    }, []);

    const handleImageChange = (event) => {
        const file = event.target.files[0];
        if (file) {
            setSelectedImage(file);
            setFileName(file.name);
            setName(file.name);
            const img = new Image();
            img.onload = () => {
                setCropX(img.width);
                setCropY(img.height);
            };
            img.src = URL.createObjectURL(file);
        }
    };

    const handleRotationChange = (value) => {
        setRotation(value);
        setActiveRotation(value);
    };

    const fetchGallerys = async () => {
        try {
            const response = await axios.get("http://localhost:8080/images");
            const response2 = await axios.get("http://localhost:8080/imagesByDate");
            const response3 = await axios.get("http://localhost:8080/imagesBySize_petit");

            const grouped = {
                allPhotos: response.data,
                recent: response2.data,
                smallSize: response3.data,
            };
            console.log(grouped);
            setGroupedImages(grouped);

        } catch (error) {
            console.error("Error fetching images:", error);
            setError("Unable to fetch images.");
        }
    };

    return (
        <div>
            <div className="background">
                <div className="containerImagePrincipal">
                    <div className="image-preview">
                        {selectedImage ? (
                            <img
                                src={selectedImage instanceof File ? URL.createObjectURL(selectedImage) : `data:image/jpeg;base64,${selectedImage}`}
                                alt="Preview"
                                className="preview-image"
                            />
                        ) : (
                            <p>No image selected</p>
                        )}
                    </div>

                    <label className="button-ind">
                        Choisir une image
                        <input
                            type="file"
                            accept="image/*"
                            onChange={handleImageChange}
                        />
                    </label>

                    <div className="sliders">
                        <div className="rotation-buttons">
                            <label>Rotation</label>
                            <div>
                                {[90, 180, 270, 0].map((value) => (
                                    <button
                                        key={value}
                                        className={`button-rot ${
                                            activeRotation === value ? "active" : ""
                                        }`}
                                        onClick={() => handleRotationChange(value)}
                                    >
                                        {value === 0 ? "0" : `${value}°`}
                                    </button>
                                ))}
                            </div>
                        </div>
                        <div className="slider">
                            <label>Brightness</label>
                            <div className="slider-container">
                                <input
                                    type="range"
                                    min="-100"
                                    max="100"
                                    className="slider-input"
                                    value={brightness}
                                    onChange={(e) => setBrightness(Number(e.target.value))}
                                />
                            </div>
                        </div>
                        <div className="slider">
                            <label>Resize</label>
                            <input
                                type="number"
                                value={cropX}
                                placeholder="Crop X"
                                onChange={(e) => setCropX(Number(e.target.value))}
                            />
                            <input
                                type="number"
                                value={cropY}
                                placeholder="Crop Y"
                                onChange={(e) => setCropY(Number(e.target.value))}
                            />
                        </div>
                    </div>

                    <input
                        type="text"
                        value={name}
                        placeholder={fileName || "Enter name"}
                        onChange={(e) => setName(e.target.value)}
                        className="name-input"
                    />

                    <div className="boutton-div">
                        <button onClick={handleTest} className="button-ind">Tester</button>
                        <button onClick={handleSubmit} className="button-ind">Add to Gallery</button>
                    </div>

                    {message && <p>{message}</p>}
                </div>
            </div>
            <div>
                <h1 className="bg-titre">Historique</h1>
                <div className="backgroundHistorique">
                    {error && <p>{error}</p>}
                    {["recent", "smallSize", "allPhotos"].map((category) => (
                        <div key={category}>
                            <h2>
                                {category === "recent"
                                    ? "Récemment ajoutées"
                                    : category === "smallSize"
                                        ? "Petite taille"
                                        : "Toutes les photos"}
                            </h2>
                            <div className="gallery-container">
                                {groupedImages[category]?.length > 0 ? (
                                    groupedImages[category].map((item, index) => (
                                        <div key={index} className="gallery-card"
                                        onClick={handleImageClick.bind(null, item.id)}
                                        >
                                            <img
                                                src={`data:image/jpeg;base64,${item.image}`}
                                                alt={item.name}
                                                className="gallery-image"
                                            />
                                            <div className="gallery-info">
                                                <p>{item.name}</p>
                                            </div>
                                        </div>
                                    ))
                                ) : (
                                    <p>Aucune image dans cette catégorie</p>
                                )}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
            {selectedImageId && (
                <ImageDetails
                    imageId={selectedImageId}
                    onClose={closeImageDetails}
                />
            )}
        </div>
    );
    {
        selectedImageId && (
            <ImageDetails
                imageId={selectedImageId}
                onClose={closeImageDetails}
            />
        )
        ;
    }
}

export default App;
