import { ProjectProp } from "../types";
import { useImageContainer } from "../hooks";
import "../static/ProjectCard.css";

export default function ProjectCard({ project }: ProjectProp) {
  let imageContainer = useImageContainer(project);

  return (
    <div
      className="projectCard"
      onClick={(_) => (window.location.href = `/project/${project.id}`)}
    >
      <h2>{project.name}</h2>
      {imageContainer}
      <p>{project.description}</p>
    </div>
  );
}
