# Powermove Terrain Gen
`Terrain Gen` is (currently) a simple yet specialized tool used to generate terrain geometry suitable for the Powermove Game.

The procedure is simple: specify "surface points" and the tool generates all relevant information.

- The tool assumes vertexes resemble the surface of the terrain and simply connects the dots to form triangles.
- As terrain, these faces have normals pointing upward-ish.
- Their UV coordinates are essentially projected along the Y axis to fit tightly into the bounds.
- Powermove operates on X - Right, Y - Up, and Z - Forward.

# TODO
- [ ] Parameterized procedural generation
- [ ] Generate UV skeleton image.
- [ ] Nudge UVs to accomodate for vertical terrain differences.
- [ ] Subterrain island geometry
