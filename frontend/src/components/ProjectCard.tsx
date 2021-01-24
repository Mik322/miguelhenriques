import { ProjectProp } from "../types";
import { useImageContainer } from "../hooks";
import { Link } from "react-router-dom";
import "../static/ProjectCard.css";

export default function ProjectCard({ project }: ProjectProp) {
  let imageContainer = useImageContainer(project);

  return (
    <div>
      <Link className="project-link" to={`/project/${project.id}`}>
        <div className="projectCard">
          <h2>{project.name}</h2>
          {imageContainer}
          <p>{project.description}</p>
        </div>
      </Link>
    </div>
  );
}
