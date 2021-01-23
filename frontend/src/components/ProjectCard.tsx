import {useEffect, useState} from "react";
import {ProjectProp} from "../types";
import "../static/ProjectCard.css"

export default function ProjectCard({project: {imageName="", name, description}}: ProjectProp) {

    const [imageContainer, setImageContainer] = useState<JSX.Element>(<></>);

    useEffect(()=> {
        import(`../static/images/${imageName}`)
            .then(img => {
                const container = createImageContainer(img);
                setImageContainer(container);
            })
            .catch(_ => {})
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    return (
        <div className="projectCard" >
            <h2>{name}</h2>
            {imageContainer}
            <p>{description}</p>
        </div>
    )
}

function createImageContainer(image: any): JSX.Element {
    return (
        <div className="imageContainer" >
            <img src={image.default} alt="" />
        </div>
    )
}