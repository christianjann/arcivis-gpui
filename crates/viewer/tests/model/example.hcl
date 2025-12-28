# Example Vehicle Instances
# References parts from VehicleSystem package in vehicle.hcl

import {
  source = "./vehicle.hcl"
}

# Sports Car Instance
vehicle "SportsCar" {
  description = "High-performance sports coupe"
  base        = "VehicleSystem.Vehicle"
  
  mass     = 1350.0
  maxSpeed = 280.0
  vin      = "WDB1234567890SPRT"

  # Powertrain configuration
  part "powertrain" {
    type = "VehicleSystem.Powertrain"
    
    power  = 350.0
    torque = 500.0

    part "engine" {
      type = "VehicleSystem.Engine"
      
      displacement = 3.0
      cylinders    = 6
      fuelType     = "gasoline"
    }
    
    part "transmission" {
      type  = "VehicleSystem.Transmission"
      gears = 7
      kind  = "dct"
    }
    
    part "driveshaft" {
      type = "VehicleSystem.Driveshaft"
    }
  }

  # Chassis configuration
  part "chassis" {
    type = "VehicleSystem.Chassis"
    
    wheelbase  = 2650.0
    trackWidth = 1620.0

    part "frontSuspension" {
      type       = "VehicleSystem.Suspension"
      kind       = "double_wishbone"
      springRate = 45.0
      damperRate = 4.5
    }
    
    part "rearSuspension" {
      type       = "VehicleSystem.Suspension"
      kind       = "multi_link"
      springRate = 50.0
      damperRate = 5.0
    }
    
    part "wheels" {
      type         = "VehicleSystem.Wheel"
      multiplicity = 4
      
      diameter    = 19.0
      width       = 255.0
      tireProfile = 35
    }
    
    part "brakes" {
      type       = "VehicleSystem.BrakeSystem"
      kind       = "disc"
      absEnabled = true

      part "frontBrakes" {
        type          = "VehicleSystem.BrakeCaliper"
        multiplicity  = 2
        pistons       = 6
        rotorDiameter = 380.0
      }
      
      part "rearBrakes" {
        type          = "VehicleSystem.BrakeCaliper"
        multiplicity  = 2
        pistons       = 4
        rotorDiameter = 350.0
      }
      
      part "absController" {
        type = "VehicleSystem.ABSController"
      }
    }
    
    part "steering" {
      type  = "VehicleSystem.SteeringSystem"
      kind  = "electric"
      ratio = 12.5
    }
  }

  # Body configuration
  part "body" {
    type      = "VehicleSystem.Body"
    doors     = 2
    seats     = 2
    bodyStyle = "coupe"

    part "exterior" {
      type = "VehicleSystem.Exterior"

      part "frontBumper" {
        type     = "VehicleSystem.Bumper"
        material = "carbon_fiber"
      }
      
      part "rearBumper" {
        type     = "VehicleSystem.Bumper"
        material = "carbon_fiber"
      }
      
      part "headlights" {
        type         = "VehicleSystem.Light"
        multiplicity = 2
        kind         = "laser"
        power        = 45.0
      }
      
      part "taillights" {
        type         = "VehicleSystem.Light"
        multiplicity = 2
        kind         = "led"
      }
      
      part "mirrors" {
        type         = "VehicleSystem.Mirror"
        multiplicity = 2
        heated       = true
        autoFolding  = true
      }
    }
    
    part "interior" {
      type = "VehicleSystem.Interior"

      part "dashboard" {
        type = "VehicleSystem.Dashboard"

        part "instrumentCluster" {
          type       = "VehicleSystem.InstrumentCluster"
          kind       = "digital"
          screenSize = 12.3
        }
        
        part "infotainment" {
          type         = "VehicleSystem.InfotainmentSystem"
          screenSize   = 12.0
          appleCarPlay = true
          androidAuto  = true
        }
      }
      
      part "seats" {
        type         = "VehicleSystem.Seat"
        multiplicity = 2
        material     = "alcantara"
        heated       = true
        ventilated   = true
        powerAdjust  = true
      }
      
      part "hvac" {
        type        = "VehicleSystem.HVACSystem"
        zones       = 2
        autoClimate = true
      }
    }
  }

  # Electrical system configuration
  part "electricalSystem" {
    type    = "VehicleSystem.ElectricalSystem"
    voltage = 12.0

    part "battery" {
      type     = "VehicleSystem.Battery"
      capacity = 80.0
      kind     = "agm"
    }
    
    part "alternator" {
      type   = "VehicleSystem.Alternator"
      output = 180.0
    }
    
    part "wiring" {
      type = "VehicleSystem.WiringHarness"
    }
    
    part "ecu" {
      type           = "VehicleSystem.ECU"
      processorSpeed = 400.0
      memory         = 8.0
    }
    
    part "sensors" {
      type = "VehicleSystem.SensorArray"
    }
  }
}


# Family SUV Instance
vehicle "FamilySUV" {
  description = "Comfortable family sport utility vehicle"
  base        = "VehicleSystem.Vehicle"
  
  mass     = 2100.0
  maxSpeed = 210.0
  vin      = "WDB9876543210SUVY"

  # Powertrain configuration
  part "powertrain" {
    type = "VehicleSystem.Powertrain"
    
    power  = 190.0
    torque = 400.0

    part "engine" {
      type = "VehicleSystem.Engine"
      
      displacement = 2.5
      cylinders    = 4
      fuelType     = "hybrid"
    }
    
    part "transmission" {
      type  = "VehicleSystem.Transmission"
      gears = 8
      kind  = "automatic"
    }
    
    part "driveshaft" {
      type = "VehicleSystem.Driveshaft"
    }
  }

  # Chassis configuration
  part "chassis" {
    type = "VehicleSystem.Chassis"
    
    wheelbase  = 2950.0
    trackWidth = 1680.0

    part "frontSuspension" {
      type       = "VehicleSystem.Suspension"
      kind       = "macpherson"
      springRate = 38.0
      damperRate = 3.5
    }
    
    part "rearSuspension" {
      type       = "VehicleSystem.Suspension"
      kind       = "multi_link"
      springRate = 40.0
      damperRate = 3.8
    }
    
    part "wheels" {
      type         = "VehicleSystem.Wheel"
      multiplicity = 4
      
      diameter    = 20.0
      width       = 265.0
      tireProfile = 50
    }
    
    part "brakes" {
      type       = "VehicleSystem.BrakeSystem"
      kind       = "regenerative"
      absEnabled = true

      part "frontBrakes" {
        type          = "VehicleSystem.BrakeCaliper"
        multiplicity  = 2
        pistons       = 4
        rotorDiameter = 350.0
      }
      
      part "rearBrakes" {
        type          = "VehicleSystem.BrakeCaliper"
        multiplicity  = 2
        pistons       = 2
        rotorDiameter = 320.0
      }
      
      part "absController" {
        type = "VehicleSystem.ABSController"
      }
    }
    
    part "steering" {
      type  = "VehicleSystem.SteeringSystem"
      kind  = "electro_hydraulic"
      ratio = 16.0
    }
  }

  # Body configuration
  part "body" {
    type      = "VehicleSystem.Body"
    doors     = 5
    seats     = 7
    bodyStyle = "suv"

    part "exterior" {
      type = "VehicleSystem.Exterior"

      part "frontBumper" {
        type     = "VehicleSystem.Bumper"
        material = "plastic"
      }
      
      part "rearBumper" {
        type     = "VehicleSystem.Bumper"
        material = "plastic"
      }
      
      part "headlights" {
        type         = "VehicleSystem.Light"
        multiplicity = 2
        kind         = "led"
        power        = 40.0
      }
      
      part "taillights" {
        type         = "VehicleSystem.Light"
        multiplicity = 2
        kind         = "led"
      }
      
      part "mirrors" {
        type         = "VehicleSystem.Mirror"
        multiplicity = 2
        heated       = true
        autoFolding  = true
      }
    }
    
    part "interior" {
      type = "VehicleSystem.Interior"

      part "dashboard" {
        type = "VehicleSystem.Dashboard"

        part "instrumentCluster" {
          type       = "VehicleSystem.InstrumentCluster"
          kind       = "hybrid"
          screenSize = 10.25
        }
        
        part "infotainment" {
          type         = "VehicleSystem.InfotainmentSystem"
          screenSize   = 11.0
          appleCarPlay = true
          androidAuto  = true
        }
      }
      
      part "seats" {
        type         = "VehicleSystem.Seat"
        multiplicity = 7
        material     = "leather"
        heated       = true
        ventilated   = false
        powerAdjust  = true
      }
      
      part "hvac" {
        type        = "VehicleSystem.HVACSystem"
        zones       = 3
        autoClimate = true
      }
    }
  }

  # Electrical system configuration
  part "electricalSystem" {
    type    = "VehicleSystem.ElectricalSystem"
    voltage = 48.0

    part "battery" {
      type     = "VehicleSystem.Battery"
      capacity = 95.0
      kind     = "lithium"
    }
    
    part "alternator" {
      type   = "VehicleSystem.Alternator"
      output = 200.0
    }
    
    part "wiring" {
      type        = "VehicleSystem.WiringHarness"
      totalLength = 3200.0
    }
    
    part "ecu" {
      type           = "VehicleSystem.ECU"
      processorSpeed = 300.0
      memory         = 6.0
    }
    
    part "sensors" {
      type = "VehicleSystem.SensorArray"
    }
  }
}


# Comparison analysis between the two vehicles
analysis "vehicle_comparison" {
  description = "Compare performance metrics between SportsCar and FamilySUV"
  
  input "sports_mass" {
    from = "SportsCar.mass"
  }
  
  input "sports_power" {
    from = "SportsCar.powertrain.power"
  }
  
  input "suv_mass" {
    from = "FamilySUV.mass"
  }
  
  input "suv_power" {
    from = "FamilySUV.powertrain.power"
  }
  
  output "sports_power_to_weight" {
    expression = "sports_power / sports_mass * 1000"
    unit       = "W/kg"
  }
  
  output "suv_power_to_weight" {
    expression = "suv_power / suv_mass * 1000"
    unit       = "W/kg"
  }
  
  output "power_ratio" {
    expression = "sports_power / suv_power"
  }
  
  output "weight_difference" {
    expression = "suv_mass - sports_mass"
    unit       = "kg"
  }
}
