import React, { useEffect, useState } from "react";

function ImageDetails({ imageId, onClose }) {
    const [imageDetails, setImageDetails] = useState(null);
    const [error, setError] = useState(null);

    useEffect(() => {
        const fetchImage = async () => {
            try {
                const response = await fetch(`http://localhost:8080/getImages/${imageId}`);
                if (response.ok) {
                    const imageData = await response.json();
                    setImageDetails(imageData);
                } else {
                    setError("Failed to fetch image details.");
                }
            } catch (err) {
                setError("An error occurred while fetching image details.");
            }
        };

        fetchImage();
    }, [imageId]);

    if (error) {
        return <div className="error">{error}</div>;
    }

    if (!imageDetails) {
        return <div>Loading image details...</div>;
    }

    return (
        <div className="image-details-modal">
            <div className="image-details-content">
                <button className="close-button" onClick={onClose}>Close</button>
                <img
                    src={`data:image/jpeg;base64,${imageDetails.image}`}
                    alt={imageDetails.name}
                    style={{ maxWidth: "100%", maxHeight: "100%" }}
                />
                <h3>{imageDetails.name}</h3>
                <p>Rotation: {imageDetails.rotation}°</p>
                <p>Brightness: {imageDetails.brightness}%</p>
                <p>Dimensions: {imageDetails.crop_x}x{imageDetails.crop_y}</p>
            </div>
        </div>
    );
}

export default ImageDetails;
