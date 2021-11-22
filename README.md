# 座 - SUWARU - **Su**rface **Wa**ter **Ru**noff: 
*A simple hydraulic rainwater runoff simulation software that runs on a gridded mesh.*

## ❓ How it works

### 🐎 Initialization
The DEM (Digital Elevation Model) and the starting water depth condition are imported into a data structure. Soon(TM) it will be possible to give each Cell a separate rain-timeseries.

### 🏇 Simulation
For each timestep, each Cells 8 neighbours are evaluated. If their water level is lower, water volume will be distributed to them according to the slope. For each of these lower neighbours, the potential water flow is calculated by using the Gauckler-Manning-Strickler (GMS) formula. At the moment, the momentum is ignored. This is a very very simplified method and is something between a simple topographical flow analysis and a fully fledged 2d hydronumeric simulation. 

## 💧💻 Hydraulic Simulations: 2D surface water modeling
Hydronumeric simulations make it possible to predict (flash) floods and the resulting flooded areas. They can be used for risk analysis and preemptive planning.

### 🌍🛩️ DEM - Digital Elevation Models
A DEM is a representation of the earths bare ground (no houses, no trees, no objects, etc.). They are often created by postprocessing aerial laser scanning into a gridded/rasterized structure. 

### 🧩🕸️ Structured Versus Unstructured Meshs
This simulation software uses a strucuted approach to modeling. The mesh is a grid of squares instead of many triangles (unstructured mesh) and getting the neighbours of each Cell is trivial. If the goal of the simulation is to find out endangered areas, this should be enough. It is also easy to parallelize, especially since the goal is to keep the problem embarassingly parallel.

### 🐟 Gauckler-Manning-Strickler 
v = 
