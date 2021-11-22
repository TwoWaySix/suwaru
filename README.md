# 座 - SUWARU - **Su**rface **Wa**ter **Ru**noff: 
*A simple hydraulic rainwater runoff simulation software that runs on a gridded mesh.*

## ❓ How it works

### 🐎 Initialization
First the DEM (Digital Elevation Model) and the starting conditions (water depth) are imported. Every xyz-Point of the DEM is converted to a Cell. Then a global refinement can be soon(TM) be done. In the future it will be possible to give each Cell a separate rain timeseries.

### 🏇 Simulation
For each timestep, each Cells' 8 neighbours are evaluated. If their water level is lower, water volume will be redistributed to them according to the slope. For each of these lower neighbours, the potential water flow is calculated using the Gauckler-Manning-Strickler (GMS) formula. At the moment, the momentum is ignored. 
This is a very very simplified method and is therefore something in between a simple topographical flow analysis and a fully fledged 2d hydronumeric simulation. 

## 💧💻 Hydraulic Simulations: 2D surface water modeling
Hydronumeric simulations make it possible to predict (flash) floods and calculate the resulting flooded areas. They can be used for risk analysis and preemptive planning. Every person should be enabled to know their risk and in many countries calculated flood areas exist at least for the near riverbed area. Simulating the rainwater surface runoff is still kind of a new discipline. Therefore most people do and can not know their risk./

### 🌍🛩️ DEM - Digital Elevation Models
A DEM is a representation of the earths bare ground (no houses, no trees, no objects, etc.). They are often created by postprocessing aerial laser scanning into a gridded/rasterized structure. 

### 🧩🕸️ Structured Versus Unstructured Meshs
This simulation software uses a strucuted approach to modeling. The mesh is a grid of equal sized squares instead of many triangles (unstructured mesh) and getting the neighbours of each Cell is trivial. If the goal of the simulation is to find  endangered areas, this should be good enough. It is also easy to parallelize, especially since the goal is to keep the problem embarassingly parallel. Parallelization still has to be implemented though...

### 🐟 Gauckler-Manning-Strickler 
v = 
