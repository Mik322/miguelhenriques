import { useEffect, useState } from "react";
import { Project } from "./types";
import { getProjects } from "./api";

export function useProjects() {
  const [projects, setProjects] = useState<Array<Project>>([]);

  useEffect(() => {
    getProjects().then((projects) => setProjects(projects));
  }, []);

  return projects;
}

export function useImageContainer(project: Project | null): JSX.Element {
  let [container, setContainer] = useState<JSX.Element>(<></>);

  useEffect(() => {
    if (project !== null) {
      import(`./static/images/${project.image_name}`)
        .then((img) => {
          setContainer(
            <div className="imageContainer">
              <img src={img.default} alt="" />
            </div>
          );
        })
        .catch((_) => {});
    }
  });

  return container;
}
