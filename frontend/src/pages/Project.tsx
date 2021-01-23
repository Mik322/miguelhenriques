import React from "react";
import {useParams} from "react-router-dom";
import {ProjectPath} from "../types"

export default function Project() {
    let { projectID } = useParams<ProjectPath>();

    return (
        <div>
            <p>{projectID}</p>
        </div>
    )
}