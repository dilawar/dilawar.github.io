---
title: "Simulation of PN junction at equilibrium in Scilab"
date: "2010-10-08"
tags: 
  - "pn-junction"
  - "simulation"
---

**This is version 0.5 There is a bug in this implementation, since hole concentration is constant in both side of the junction. Thanks to Urmimala for pointing this out.. In ver 1.0, we'll provide complete documentation and will remove this bug.** The [pn-junction](http://ecee.colorado.edu/~bart/book/book/chapter4/ch4_2.htm) is quite an interesting junction. If most the semiconductor theory can be tested around it, it also poses significant challenges in numerical method also. In this  implementation in Scilab, we are solving Poisson equation to get the results when junction is in equilibrium. No part of this work used propriety softwares. Please wait for complete documentation of code.

# Simulation Results

\[gallery link="file" columns="5" orderby="rand"\] Click on image to enlarge it.

# Scilab Functions

This is my top most function. Write a script which call this function and every other functions will be called in order. \[sourcecode language="matlab"\] // This function will discretized the domain and will create a grid points. // Then we'll discretise the equation there. Matrices // will be created and saved in an binary file for further processing. function \[\] = solveEquilibriumCondition() // Open file to read the data. // Check if file exists. If not run the script to create it. ifExists = exists("./profilePN.dat"); if ifExists == 0 then disp("File does not exist. Executing script to generate this file."); exec("./profilePN.sci"); profilePN(); load("./profilePN.dat"); load("./myParameters.dat"); else disp('File already exists. Loading ...'); load("./profilePN.dat"); load("./myParameters.dat"); disp("if there is any problem with this file. Delete it and re-execute this function/script."); end; delta\_acc = 1E-5;               // Preset the Tolerance //////////////////////////////////////////////////////////////////////// /////////// EQUILIBRIUM  SOLUTION PART BEGINS                      ///// //////////////////////////////////////////////////////////////////////// // Define the elements of the coefficient matrix for the internal nodes and dx2 = delX\*delX; for i = 2: n\_max-1 a(i) = 1/dx2; c(i) = 1/dx2; b(i) = -(2/dx2+exp(fi(i))+exp(-fi(i))); f(i) = exp(fi(i)) - exp(-fi(i)) - dop(i) - fi(i)\*(exp(fi(i))+exp(-fi(i))); end // Define the elements of the coefficient matrix and initialize the forcing //   function at the ohmic contacts a(1) = 0; c(1) = 0; b(1) = 1; f(1) = fi(1); a(n\_max) = 0; c(n\_max) = 0; b(n\_max) = 1; f(n\_max) = fi(n\_max); //  Start the iterative procedure for the solution of the linearized Poisson //  equation using LU decomposition method: flag\_conv = 0;                   // convergence of the Poisson loop k\_iter= 0; while flag\_conv == 0 then k\_iter = k\_iter + 1; alpha1(1) = b(1); for i= 2 : n\_max beta1(i)=a(i)/alpha1(i-1); alpha1(i)=b(i)-beta1(i)\*c(i-1); end; // Solution of Lv = f v(1) = f(1); for i = 2:n\_max v(i) = f(i) - beta1(i)\*v(i-1); end; // Solution of U\*fi = v temp = v(n\_max)/alpha1(n\_max); delta(n\_max) = temp - fi(n\_max); fi(n\_max)=temp; for i = (n\_max-1):-1:1       //delta temp = (v(i)-c(i)\*fi(i+1))/alpha1(i); delta(i) = temp - fi(i); fi(i) = temp; end; delta\_max = 0; for i = 1: n\_max xx = abs(delta(i)); if xx > delta\_max then delta\_max=xx; end; //sprintf('delta\_max = %d',delta\_max)      //'k\_iter = %d',k\_iter,' end; //delta\_max=max(abs(delta)); // Test convergence and recalculate forcing function and // central coefficient b if necessary if delta\_max < delta\_acc then flag\_conv = 1; else for i = 2: n\_max-1 b(i) = -(2/dx2 + exp(fi(i)) + exp(-fi(i))); f(i) = exp(fi(i)) - exp(-fi(i)) - dop(i) - fi(i)\*(exp(fi(i)) + exp(-fi(i))); end; end; end; xx1(1) = delX\*1e4; for i = 2:n\_max-1 Ec(i) = dEc - Vt\*fi(i);     //Values from the second Node% ro(i) = -ni\*(exp(fi(i)) - exp(-fi(i)) - dop(i)); el\_field1(i) = -(fi(i+1) - fi(i))\*Vt/(delX\*Ldi); el\_field2(i) = -(fi(i+1) - fi(i-1))\*Vt/(2\*delX\*Ldi); n(i) = exp(fi(i)); p(i) = exp(-fi(i)); xx1(i) = xx1(i-1) + delX\*Ldi\*1e4; end; Ec(1) = Ec(2); Ec(n\_max) = Ec(n\_max-1); xx1(n\_max) = xx1(n\_max-1) + delX\*Ldi\*1e4; el\_field1(1) = el\_field1(2); el\_field2(1) = el\_field2(2); el\_field1(n\_max) = el\_field1(n\_max-1); el\_field2(n\_max) = el\_field2(n\_max-1); nf = n\*ni; pf = p\*ni; ro(1) = ro(2); ro(n\_max) = ro(n\_max-1); // save all these values in a file. save('equilibriumSol.dat', Ec, Vt, fi, xx1, nf, pf, ro, n, p, el\_field1, el\_field2); h = figure(1) plot(xx1, Vt\*fi,'r','LineWidth',2) xlabel('x \[um\]'); ylabel('Potential \[eV\]'); title('Potential vs Position - at Equilibrium'); xs2gif(1, './figs/PotentialVsPos.gif'); close(h); h = figure(2) plot(xx1, \[el\_field1, el\_field2\],'r','LineWidth',2) //plot(xx1, el\_field2,'r','LineWidth',2) xlabel('x \[um\]'); ylabel('Electric Field \[V/cm\]'); title('Field Profile vs Position - at Equilibrium'); xs2gif(2, './figs/ElectricalFieldVsPos.gif'); close(h); h = figure(3) plot2d1("onn", xx1, \[nf, pf\]); xlabel('x \[um\]'); ylabel('Electron & Hole Densities \[1/cm^3\]'); title('Electron & Hole Densities vs Position - at Equilibrium'); legend('n','p'); xs2gif(3, './figs/ElectronAndHolesDensity.gif'); close(h); h = figure(4) plot(xx1, q\_e\*ro,'r','LineWidth',2) xlabel('x \[um\]'); ylabel('Total Charge Density \[C/cm^3\]'); title('Total Charge Density vs Position - at Equilibrium'); xs2gif(4, './figs/TotalChargeDensity.gif'); close(h); h = figure(5) plot(xx1, Ec,'r','LineWidth',2) xlabel('x \[um\]'); ylabel('Conduction Band Energy (eV)'); title('Conduction Band vs Position - at Equilibrium'); xs2gif(5, './figs/ConductionBandDiagram.gif'); close(h); endfunction \[/sourcecode\]

This function will profile the PN junction and return the doping and p and n distributions across the junction
\[sourcecode language="matlab"\]
// This function will profile the PN junction and return the doping and p and n
// distributions across the junction.
// Open file to read the data.
// Check if file exists. If not run the script to create it.
function \[\] = profilePN()
  ifExists = exists("./myParameters.dat");
  if ifExists == 0 then
    disp("File does not exist. Executing script to generate this file.");
    exec("./calcParameters.sci");
    calcParameters();
    load("./myParameters.dat");
   else
    disp('File already exists. Loading ...');
    load("./myParameters.dat");
    disp("if there is any problem with this file. Delete it and re-execute this function/script.");
  end;
  // Now we need to discretise the boundries.
  // Set grid size based on the extrinsic Debye Lengths.
  if Ldn < Ldp then
    delX = Ldn/10;
  else
    delX = Ldp/10;
  end;
  // Length of the pn junction.
  if Wp > Wn then
    length\_pn = 30\*Wn;
  else
    length\_pn = 30\*Wp;
  end;
  // We have grid size, We have length. Calculate the total points.
  n\_max = floor(length\_pn/delX);
  delX = delX/Ldi; // renormalize lengths with Ldi.
  // Set up doping C(x) = Nd(x) - Na(x) that is normalised with ni
  // Half of the point are p type, rests are n-types.
  for i = 1:n\_max
    if i <= n\_max/2 then
      dop(i) = -Na/ni; // p type
    elseif i > n\_max/2 then
      dop(i) = Nd/ni; // n type
    end
  end
  // Initialize the potential based on the requirement of charge neutrality
  // throughout the whole structure.
  for i = 1: n\_max
    tempZ = 0.5\*dop(i);
    if tempZ > 0 then
      tempX = tempZ \* (1 + sqrt(1 + 1/(tempZ\*tempZ)));
    elseif tempZ < 0 then
      tempX = tempZ \* (1 - sqrt( 1 + 1/(tempZ\*tempZ)));
    end;
    fi(i) = log(tempX);
    n(i) = tempX;
    p(i) = 1/tempX;
  end;
  save('profilePN.dat', fi, p, n, dop, length\_pn, n\_max, delX);
endfunction
\[/sourcecode\]
 Here is the part which calculates the parameters to be used by above posted functions. \[sourcecode language="matlab"\]
// This function will calculate the parameters. We'll store these parameters
// in a file. We'll use this file to reuse the parametes for further processing.
// (c) Dilawar, 2010
// dilawar@ee.iitb.ac.in
// www.ee.iitb.ac.in/student/~dilawar
// GNU - GPL
// This function should return a ascii file containing parameters which are to
// be used later.
function \[\] = calcParameters()
T = 300;           // Substrate is Silicon. Temp 300 K
u\_e = 1350;        // cm\*cm/V/s Mobility of electron in Silicon
u\_p = 480;         // cm\*cm/V/s  Mobility of holes in Silicon
D\_n = 35;          // cm\*cm/s   Diffusion coeff.
D\_p = 12.4;        // cm\*cm/s  Diffusion coeff.
q\_e = 1.6e-19;     // Charge on an electron.
kt\_q = 0.0259;
kb = 1.38e-23;     // Boltzmann Constant JK^-1
eps = 1.05e-12;    // Permittivity of undopes silicon.
ni = 1.5e10;       // Intrinsic carrier concentration
Vt = kb\*T/q\_e;     // Threshold voltages.
RNc = 2.8e20;
dEc = Vt\*log(RNc/ni);
tauN0 = 0.1e-6;
tauP0 = 0.1e-6;
mu\_n0 = 1500;
mu\_p0 = 1000;
// Define doping values.
Na = 1e16;
Nd = 1e18;
// Calculate parameters.
Vbi = Vt\*log(Na\*Nd/(ni\*ni));                // Built-in voltage.
W = sqrt(2\*eps\*(Na + Nd)\*Vbi/(q\_e\*Na\*Nd) ); // Depletion width
Wn = W\*sqrt(Na/(Na + Nd));                  // Depletion width at n side.
Wp = W\*sqrt(Nd/(Na + Nd));                  // Depletion width at p side
Wone = sqrt(2\*eps\*Vbi/(q\_e\*Na));            //
E\_p = q\_e\*Nd\*Wn/eps;
Ldn = sqrt(eps\*Vt/(q\_e\*Nd));
Ldp = sqrt(eps\*Vt/(q\_e\*Na));
Ldi = sqrt(eps\*Vt/(q\_e\*ni));
// We'd like to save our paramters in some file.
save('myParameters.dat', q\_e, Na, Nd, Vbi, W, Wn, Wp, E\_p, Ldn, Ldp, Ldi, ni, dEc, Vt, kt\_q, mu\_n0, mu\_p0);
endfunction;
\[/sourcecode\]

// This function will discretize the domain and will create a grid points.  // Then we'll discretise the equation there. Matrices  // will be created and saved in an binary file for further processing.    function \[\] = solveEquilibriumCondition()  // Open file to read the data.  // Check if file exists. If not run the script to create it.  ifExists = exists("./profilePN.dat");  if ifExists == 0 then  disp("File does not exist. Executing script to generate this file.");  exec("./profilePN.sci");  profilePN();  load("./profilePN.dat");  load("./myParameters.dat");  else  disp('File already exists. Loading ...');  load("./profilePN.dat");  load("./myParameters.dat");  disp("if there is any problem with this file. Delete it and re-execute this function/script.");  end;  delta\_acc = 1E-5;               // Preset the Tolerance  ////////////////////////////////////////////////////////////////////////  /////////// EQUILIBRIUM  SOLUTION PART BEGINS                      /////  ////////////////////////////////////////////////////////////////////////  // Define the elements of the coefficient matrix for the internal nodes and  dx2 = delX\*delX;  for i = 2: n\_max-1  a(i) = 1/dx2;  c(i) = 1/dx2;  b(i) = -(2/dx2+exp(fi(i))+exp(-fi(i)));  f(i) = exp(fi(i)) - exp(-fi(i)) - dop(i) - fi(i)\*(exp(fi(i))+exp(-fi(i)));  end  // Define the elements of the coefficient matrix and initialize the forcing  //   function at the ohmic contacts  a(1) = 0;  c(1) = 0;  b(1) = 1;  f(1) = fi(1);  a(n\_max) = 0;  c(n\_max) = 0;  b(n\_max) = 1;  f(n\_max) = fi(n\_max);  //  Start the iterative procedure for the solution of the linearized Poisson  //  equation using LU decomposition method:  flag\_conv = 0;                   // convergence of the Poisson loop  k\_iter= 0;  while flag\_conv == 0 then  k\_iter = k\_iter + 1;  alpha1(1) = b(1);  for i= 2 : n\_max  beta1(i)=a(i)/alpha1(i-1);  alpha1(i)=b(i)-beta1(i)\*c(i-1);  end;  // Solution of Lv = f  v(1) = f(1);  for i = 2:n\_max  v(i) = f(i) - beta1(i)\*v(i-1);  end;  // Solution of U\*fi = v  temp = v(n\_max)/alpha1(n\_max);  delta(n\_max) = temp - fi(n\_max);  fi(n\_max)=temp;  for i = (n\_max-1):-1:1       //delta  temp = (v(i)-c(i)\*fi(i+1))/alpha1(i);  delta(i) = temp - fi(i);  fi(i) = temp;  end;  delta\_max = 0;  for i = 1: n\_max  xx = abs(delta(i));  if xx > delta\_max then  delta\_max=xx;  end;  //sprintf('delta\_max = %d',delta\_max)      //'k\_iter = %d',k\_iter,'  end;  //delta\_max=max(abs(delta));  // Test convergence and recalculate forcing function and  // central coefficient b if necessary  if delta\_max < delta\_acc then  flag\_conv = 1;  else  for i = 2: n\_max-1  b(i) = -(2/dx2 + exp(fi(i)) + exp(-fi(i)));  f(i) = exp(fi(i)) - exp(-fi(i)) - dop(i) - fi(i)\*(exp(fi(i)) + exp(-fi(i)));  end;  end;  end;  xx1(1) = delX\*1e4;  for i = 2:n\_max-1  Ec(i) = dEc - Vt\*fi(i);     //Values from the second Node%  ro(i) = -ni\*(exp(fi(i)) - exp(-fi(i)) - dop(i));  el\_field1(i) = -(fi(i+1) - fi(i))\*Vt/(delX\*Ldi);  el\_field2(i) = -(fi(i+1) - fi(i-1))\*Vt/(2\*delX\*Ldi);  n(i) = exp(fi(i));  p(i) = exp(-fi(i));  xx1(i) = xx1(i-1) + delX\*Ldi\*1e4;  end;  Ec(1) = Ec(2);  Ec(n\_max) = Ec(n\_max-1);  xx1(n\_max) = xx1(n\_max-1) + delX\*Ldi\*1e4;  el\_field1(1) = el\_field1(2);  el\_field2(1) = el\_field2(2);  el\_field1(n\_max) = el\_field1(n\_max-1);  el\_field2(n\_max) = el\_field2(n\_max-1);  nf = n\*ni;  pf = p\*ni;  ro(1) = ro(2);  ro(n\_max) = ro(n\_max-1);  // save all these values in a file.  save('equilibriumSol.dat', Ec, Vt, fi, xx1, nf, pf, ro, n, p, el\_field1, el\_field2);  h = figure(1)  plot(xx1, Vt\*fi,'r','LineWidth',2)  xlabel('x \[um\]');  ylabel('Potential \[eV\]');  title('Potential vs Position - at Equilibrium');  xs2gif(1, './figs/PotentialVsPos.gif');  close(h);  h = figure(2)  plot(xx1, \[el\_field1, el\_field2\],'r','LineWidth',2)  //plot(xx1, el\_field2,'r','LineWidth',2)  xlabel('x \[um\]');  ylabel('Electric Field \[V/cm\]');  title('Field Profile vs Position - at Equilibrium');  xs2gif(2, './figs/ElectricalFieldVsPos.gif');  close(h);  h = figure(3)  plot2d1("onn", xx1, \[nf, pf\]);  xlabel('x \[um\]');  ylabel('Electron & Hole Densities \[1/cm^3\]');  title('Electron & Hole Densities vs Position - at Equilibrium');  legend('n','p');  xs2gif(3, './figs/ElectronAndHolesDensity.gif');  close(h);  h = figure(4)  plot(xx1, q\_e\*ro,'r','LineWidth',2)  xlabel('x \[um\]');  ylabel('Total Charge Density \[C/cm^3\]');  title('Total Charge Density vs Position - at Equilibrium');  xs2gif(4, './figs/TotalChargeDensity.gif');  close(h);  h = figure(5)  plot(xx1, Ec,'r','LineWidth',2)  xlabel('x \[um\]');  ylabel('Conduction Band Energy (eV)');  title('Conduction Band vs Position - at Equilibrium');  xs2gif(5, './figs/ConductionBandDiagram.gif');  close(h);  endfunction