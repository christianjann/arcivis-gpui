package VehicleModel {
    
    part def Vehicle {
        attribute mass : Real;
        attribute maxSpeed : Real;
        
        part engine : Engine;
        part wheels : Wheel[4];
        part transmission : Transmission;
    }
    
    part def Engine {
        attribute horsepower : Real;
        attribute displacement : Real;
    }
    
    part def Wheel {
        attribute diameter : Real;
        attribute type : String;
    }
    
    part def Transmission {
        attribute gears : Integer;
        attribute type : String;
    }
    
    part def Car :> Vehicle {
        attribute numDoors : Integer = 4;
    }
    
    part def Truck :> Vehicle {
        attribute bedLength : Real;
        attribute cargoCapacity : Real;
    }
}