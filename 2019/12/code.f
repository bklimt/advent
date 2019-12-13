
c Define our own signum functions.
      integer function signum(n)
      integer n
      if (n.lt.0) signum=-1
      if (n.eq.0) signum=0
      if (n.gt.0) signum=1
      return
      end

c A program to track orbits.
      program advent
      integer signum
      integer i,j
      integer t
      integer x(4),   y(4),  z(4)
      integer dx(4), dy(4), dz(4)
      integer e, pot, kin

c Hard-coded initial data.
      data x/ -16,   0, -11,   2/
      data y/  -1,  -4,  11,   2/
      data z/ -12, -17,   0,  -6/
      data dx/4*0/, dy/4*0/, dz/4*0/

c Hard-coded first example.
c     data x/  -1,   2,   4,   3/
c     data y/   0, -10,  -8,   5/
c     data z/   2,  -7,   8,  -1/
c     data dx/4*0/, dy/4*0/, dz/4*0/

c Loop over a fixed number of time steps.
      do 10 t=0,1000

c Print the current state of the universe.
      write (*,100) 't=', t
 100  format (a,i4)
      do 20 j=1,4
      write (*,200) 'pos=<x=',  x(j), ', y=',  y(j), ', z=',  z(j),
     +           '>, vel=<x=', dx(j), ', y=', dy(j), ', z=', dz(j), '>'
 200  format (6(a,i4),a)
  20  continue

c Update the velocity of each moon.
      do 30 i=1,4
      do 40 j=1,4
      dx(i) = dx(i) + signum(x(j)-x(i))
      dy(i) = dy(i) + signum(y(j)-y(i))
      dz(i) = dz(i) + signum(z(j)-z(i))
  40  continue
  30  continue

C Update the position of each moon and compute energy.
      e = 0
      do 50 i=1,4
      x(i) = x(i) + dx(i)
      y(i) = y(i) + dy(i)
      z(i) = z(i) + dz(i)
      pot = abs( x(i)) + abs( y(i)) + abs( z(i))
      kin = abs(dx(i)) + abs(dy(i)) + abs(dz(i))
      e = e + (pot * kin)
  50  continue
      write (*,300) 'e=', e
 300  format (a,i8)

  10  continue
      stop
      end

c 12.1 = 5517
