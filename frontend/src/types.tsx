export interface Project {
    id: number,
    imageName?: String,
    name: String,
    description: String
}

export type ProjectProp = {
    project: Project,
}

export interface ProjectList {
    projects: Array<Project>,
    maxSize?: number
}

export type Email = {
    from: String,
    subject: String,
    text: String
}

export interface ProjectPath {
    projectID: string
}