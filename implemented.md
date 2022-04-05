# Implementaion overview 

This list is directly pulled from the spice documentation: https://naif.jpl.nasa.gov/pub/naif/toolkit`_`docs/C/cspice/

The first checkmark means "this function is implemented somehow" 
The second checkmark means "this function is part of the most used cspice apis" listed here: https://naif.jpl.nasa.gov/pub/naif/toolkit`_`docs/C/info/mostused.html
Implemented functions also have their rust-api and/or a note appended at the end. 

### A
[] [] `APPNDC_C` - Append an item to a character cell
[] [] `APPNDD_C` - Append an item to a double precision cell
[] [] `APPNDI_C` - Append an item to an integer cell
[] [] `AXISAR_C` - Axis and angle to rotation
[] [] `AZLCPO_C` - AZ/EL, constant position observer state
[] [] `AZLREC_C` - AZ/EL to rectangular coordinates

### B
[] [] `B1900_C` - Besselian Date 1900.0
[] [] `B1950_C` - Besselian Date 1950.0
[] [] `BADKPV_C` - Bad Kernel Pool Variable
[] [] `BLTFRM_C` - Built-in frame IDs
[] [] `BODC2N_C` - Body ID code to name translation
[] [] `BODC2S_C` - Body ID code to string translation
[] [] `BODDEF_C` - Body name/ID code definition
[] [] `BODFND_C` - Find values from the kernel pool
[] [] `BODN2C_C` - Body name to ID code translation
[] [] `BODS2C_C` - Body string to ID code translation
[] [] `BODVAR_C` - Return values from the kernel pool
[] [] `BODVCD_C` - Return d.p. values from the kernel pool
[] [] `BODVRD_C` - Return d.p. values from the kernel pool
[] [] `BRCKTD_C` - Bracket a d.p. value within an interval
[] [] `BRCKTI_C` - Bracket an integer value within an interval
[] [] `BSCHOC_C` - Binary search with order vector, character
[] [] `BSCHOI_C` - Binary search with order vector, integer
[] [] `BSRCHC_C` - Binary search for a character string
[] [] `BSRCHD_C` - Binary search for a double precision value
[] [] `BSRCHI_C` - Binary search for an integer value

### C
[] [] `CARD_C` - Cardinality of a cell
[] [] `CCIFRM_C` - Class and class ID to associated frame
[] [] `CGV2EL_C` - Center and generating vectors to ellipse
[] [] `CHBDER_C` - Derivatives of a Chebyshev expansion
[] [] `CHBIGR_C` - Chebyshev expansion integral
[] [] `CHBINT_C` - Interpolate a Chebyshev expansion
[] [] `CHBVAL_C` - Value of a Chebyshev polynomial expansion
[] [] `CHKIN_C` - module Check In
[] [] `CHKOUT_C` - Module Check Out
[] [] `CIDFRM_C` - center SPK ID frame
[] [] `CKCLS_C` - CK, Close file
[] [] `CKCOV_C` - CK coverage
[] [] `CKFROT_C` - CK frame, find position rotation
[] [] `CKFXFM_C` - CK frame, find state transformation
[] [] `CKGP_C` - C-kernel, get pointing
[] [] `CKGPAV_C` - C-kernel, get pointing and angular velocity
[] [] `CKGR02_C` - C-kernel, get record, type 02
[] [] `CKGR03_C` - C-kernel, get record, type 03
[] [] `CKLPF_C` - CK, load pointing file
[] [] `CKMETA_C` - CK ID to associated SCLK
[] [] `CKNR02_C` - C-kernel, number of records, type 02
[] [] `CKNR03_C` - C-kernel, number of records, type 03
[] [] `CKOBJ_C` - CK objects
[] [] `CKOPN_C` - CK, open new file.
[] [] `CKUPF_C` - CK, Unload pointing file
[] [] `CKW01_C` - C-Kernel, write segment to C-kernel, data type 1
[] [] `CKW02_C` - C-Kernel, write segment to C-kernel, data type 2
[] [] `CKW03_C` - C-Kernel, write segment to C-kernel, data type 3
[] [] `CKW05_C` - Write CK segment, type 5
[] [] `CLEARC_C` - Clear a two-dimensional character array
[] [] `CLEARD_C` - Clear a double precision array
[] [] `CLEARI_C` - Clear an integer array
[] [] `CLIGHT_C` - C, Speed of light in a vacuum
[] [] `CLPOOL_C` - Clear the pool of kernel variables
[] [] `CMPRSS_C` - Compress a character string
[] [] `CNMFRM_C` - Center name to associated frame
[] [] `CONICS_C` - Determine state from conic elements
[] [] `CONVRT_C` - Convert Units
[] [] `COPY_C` - Copy a SPICE cell
[] [] `CPOS_C` - Character position
[] [] `CPOSR_C` - Character position, reverse
[] [] `CVPOOL_C` - Check variable in the pool for update
[] [] `CYLLAT_C` - Cylindrical to latitudinal
[] [] `CYLREC_C` - Cylindrical to rectangular
[] [] `CYLSPH_C` - Cylindrical to spherical

### D
[] [] `DAFAC_C` - DAF add comments
[] [] `DAFBBS_C` - DAF, begin backward search
[] [] `DAFBFS_C` - DAF, begin forward search
[] [] `DAFCLS_C` - DAF, close
[] [] `DAFCS_C` - DAF, continue search
[] [] `DAFDC_C` - DAF delete comments
[] [] `DAFEC_C` - DAF extract comments
[] [] `DAFFNA_C` - DAF, find next array
[] [] `DAFFPA_C` - DAF, find previous array
[] [] `DAFGDA_C` - DAF, read data from address
[] [] `DAFGH_C` - DAF, get handle
[] [] `DAFGN_C` - DAF, get array name
[] [] `DAFGS_C` - DAF, get summary
[] [] `DAFGSR_C` - DAF, get summary/descriptor record
[] [] `DAFHSF_C` - DAF, handle to summary format
[] [] `DAFOPR_C` - DAF, open for read
[] [] `DAFOPW_C` - DAF, open for write
[] [] `DAFPS_C` - DAF, pack summary
[] [] `DAFRDA_C` - DAF, read data from address
[] [] `DAFRFR_C` - DAF, read file record
[] [] `DAFRS_C` - DAF, replace summary
[] [] `DAFUS_C` - DAF, unpack summary
[] [] `DASAC_C` - DAS add comments
[] [] `DASADC_C` - DAS, add data, character
[] [] `DASADD_C` - DAS, add data, double precision
[] [] `DASADI_C` - DAS, add data, integer
[] [] `DASCLS_C` - DAS, close file
[] [] `DASDC_C` - DAS, delete comments
[] [] `DASEC_C` - DAS extract comments
[] [] `DASHFN_C` - DAS, handle to file name
[] [] `DASHFS_C` - DAS, handle to file summary
[] [] `DASLLA_C` - DAS, last logical addresses
[] [] `DASLLC_C` - DAS, low-level close
[] [] `DASONW_C` - DAS, open new file
[] [] `DASOPR_C` - DAS, open for read
[] [] `DASOPS_C` - DAS, open scratch
[] [] `DASOPW_C` - DAS, open for write
[] [] `DASRDC_C` - DAS, read data, character
[] [] `DASRDD_C` - DAS, read data, double precision
[] [] `DASRDI_C` - DAS, read data, integer
[] [] `DASRFR_C` - DAS, read file record
[] [] `DASUDC_C` - DAS, update data, character
[] [] `DASUDD_C` - DAS, update data, double precision
[] [] `DASUDI_C` - DAS, update data, integer
[] [] `DASWBR_C` - DAS, write buffered records
[] [] `DAZLDR_C` - Derivative of AZ/EL w.r.t. rectangular
[] [] `DCYLDR_C` - Derivative of cylindrical w.r.t. rectangular
[] [] `DELTET_C` - Delta ET, ET - UTC
[] [] `DET_C` - Determinant of a double precision 3x3 matrix
[] [] `DGEODR_C` - Derivative of geodetic w.r.t. rectangular
[] [] `DIAGS2_C` - Diagonalize symmetric 2x2 matrix
[] [] `DIFF_C` - Difference of two sets
[] [] `DLABBS_C` - DLA, begin backward search
[] [] `DLABFS_C` - DLA, begin forward search
[] [] `DLABNS_C` - DLA, begin new segment
[] [] `DLAENS_C` - DLA, end new segment
[] [] `DLAFNS_C` - DLA, find next segment
[] [] `DLAFPS_C` - DLA, find previous segment
[] [] `DLAOPN_C` - DLA, open new file
[] [] `DLATDR_C` - Derivative of latitudinal w.r.t. rectangular
[] [] `DNEARP_C` - Derivative of near point
[] [] `DP2HX_C` - D.p. number to hexadecimal string
[] [] `DPGRDR_C` - Derivative of planetographic w.r.t. rectangular
[] [] `DPMAX_C` - Largest DP number
[] [] `DPMIN_C` - Smallest DP number
[] [] `DPR_C` - Degrees per radian
[] [] `DRDAZL_C` - Derivative of rectangular w.r.t. AZ/EL
[] [] `DRDCYL_C` - Derivative of rectangular w.r.t. cylindrical
[] [] `DRDGEO_C` - Derivative of rectangular w.r.t. geodetic
[] [] `DRDLAT_C` - Derivative of rectangular w.r.t. latitudinal
[] [] `DRDPGR_C` - Derivative of rectangular w.r.t. planetographic
[] [] `DRDSPH_C` - Derivative of rectangular w.r.t. spherical
[] [] `DSKB02_C` - DSK, fetch type 2 bookkeeping data
[] [] `DSKCLS_C` - DSK, close file
[] [] `DSKD02_C` - DSK, fetch d.p. type 2 data
[] [] `DSKGD_C` - DSK, return DSK segment descriptor
[] [] `DSKGTL_C` - DSK, get tolerance
[] [] `DSKI02_C` - DSK, fetch integer type 2 data
[] [] `DSKMI2_C` - DSK, make spatial index for type 2 segment
[] [] `DSKN02_C` - DSK, type 2, compute normal vector for plate
[] [] `DSKOBJ_C` - DSK, get object IDs
[] [] `DSKOPN_C` - DSK, open new file
[] [] `DSKP02_C` - DSK, fetch type 2 plate data
[] [] `DSKRB2_C` - DSK, determine range bounds for plate set
[] [] `DSKSRF_C` - DSK, get surface IDs for body
[] [] `DSKSTL_C` - DSK, set tolerance
[] [] `DSKV02_C` - DSK, fetch type 2 vertex data
[] [] `DSKW02_C` - DSK, write type 2 segment
[] [] `DSKX02_C` - DSK, ray-surface intercept, type 2
[] [] `DSKXSI_C` - DSK, ray-surface intercept with source information
[] [] `DSKXV_C` - DSK, ray-surface intercept, vectorized
[] [] `DSKZ02_C` - DSK, fetch type 2 model size parameters
[] [] `DSPHDR_C` - Derivative of spherical w.r.t. rectangular
[] [] `DTPOOL_C` - Data for a kernel pool variable
[] [] `DUCRSS_C` - Unit Normalized Cross Product and Derivative
[] [] `DVCRSS_C` - Derivative of Vector cross product
[] [] `DVDOT_C` - Derivative of Vector Dot Product, 3-D
[] [] `DVHAT_C` - Derivative and unit vector "V-hat" of a state
[] [] `DVNORM_C` - Derivative of vector norm
[] [] `DVPOOL_C` - Delete a variable from the kernel pool
[] [] `DVSEP_C` - Time derivative of separation angle

### E
[] [] `EDLIMB_C` - Ellipsoid Limb
[] [] `EDNMPT_C` - Ellipsoid normal vector to surface point
[] [] `EDPNT_C` - Ellipsoid point
[] [] `EDTERM_C` - Ellipsoid terminator
[] [] `EKACEC_C` - EK, add character data to column
[] [] `EKACED_C` - EK, add d.p. data to column
[] [] `EKACEI_C` - EK, add integer data to column
[] [] `EKACLC_C` - EK, add character column to segment
[] [] `EKACLD_C` - EK, add double precision column to segment
[] [] `EKACLI_C` - EK, add integer column to segment
[] [] `EKAPPR_C` - EK, append record onto segment
[] [] `EKBSEG_C` - EK, start new segment
[] [] `EKCCNT_C` - EK, column count
[] [] `EKCII_C` - EK, column info by index
[] [] `EKCLS_C` - EK, close file
[] [] `EKDELR_C` - EK, delete record from segment
[] [] `EKFFLD_C` - EK, finish fast write
[] [] `EKFIND_C` - EK, find data
[] [] `EKGC_C` - EK, get event data, character
[] [] `EKGD_C` - EK, get event data, double precision
[] [] `EKGI_C` - EK, get event data, integer
[] [] `EKIFLD_C` - EK, initialize segment for fast write
[] [] `EKINSR_C` - EK, insert record into segment
[] [] `EKLEF_C` - EK, load event file
[] [] `EKNELT_C` - EK, get number of elements in column entry
[] [] `EKNSEG_C` - EK, number of segments in file
[] [] `EKNTAB_C` - EK, return number of loaded tables
[] [] `EKOPN_C` - EK, open new file
[] [] `EKOPR_C` - EK, open file for reading
[] [] `EKOPS_C` - EK, open scratch file
[] [] `EKOPW_C` - EK, open file for writing
[] [] `EKPSEL_C` - EK, parse SELECT clause
[] [] `EKRCEC_C` - EK, read column entry element, character
[] [] `EKRCED_C` - EK, read column entry element, d.p.
[] [] `EKRCEI_C` - EK, read column entry element, integer
[] [] `EKSSUM_C` - EK, return segment summary
[] [] `EKTNAM_C` - EK, return name of loaded table
[] [] `EKUCEC_C` - EK, update character column entry
[] [] `EKUCED_C` - EK, update d.p. column entry
[] [] `EKUCEI_C` - EK, update integer column entry
[] [] `EKUEF_C` - EK, unload event file
[] [] `EL2CGV_C` - Ellipse to center and generating vectors
[] [] `ELEMC_C` - Element of a character set
[] [] `ELEMD_C` - Element of a double precision set
[] [] `ELEMI_C` - Element of an integer set
[] [] `EQNCPV_C` - Equinoctial Elements to position and velocity
[] [] `EQSTR_C` - Equivalent strings
[] [] `ERRACT_C` - Get/Set Default Error Action
[] [] `ERRCH_C` - Insert String into Error Message Text
[] [] `ERRDEV_C` - Get/Set Error Output Device Name
[] [] `ERRDP_C` - Insert D.P. Number into Error Message Text
[] [] `ERRINT_C` - Insert Integer into Error Message Text
[] [] `ERRPRT_C` - Get/Set Error Output Items
[] [] `ESRCHC_C` - Equivalence search, character
[] [] `ET2LST_C` - ET to Local Solar Time
[] [] `ET2UTC_C` - Ephemeris Time to UTC
[] [] `ETCAL_C` - Convert ET to Calendar format
[] [] `EUL2M_C` - Euler angles to matrix
[] [] `EUL2XF_C` - Euler angles and derivative to transformation
[] [] `EVSGP4_C` - Evaluate "two-line" element data
[] [] `EXISTS_C` - Does the file exist?
[] [] `EXPOOL_C` - Confirm the existence of a pooled kernel variable

### F
[] [] `FAILED_C` - Error Status Indicator
[] [] `FILLD_C` - Fill a double precision array
[] [] `FILLI_C` - Fill an integer array
[] [] `FOVRAY_C` - Is ray in FOV at time?
[] [] `FOVTRG_C` - Is target in FOV at time?
[] [] `FRAME_C` - Build a right handed coordinate frame
[] [] `FRINFO_C` - Frame Information
[] [] `FRMNAM_C` - Frame to Name
[] [] `FTNCLS_C` - Close file designated by Fortran unit
[] [] `FURNSH_C` - Furnish a program with SPICE kernels

### F
[] [] `GCPOOL_C` - Get character data from the kernel pool
[] [] `GDPOOL_C` - Get d.p. values from the kernel pool
[] [] `GEOREC_C` - Geodetic to rectangular coordinates
[] [] `GETCML_C` - Get the command line
[] [] `GETELM_C` - Get the components from two-line elements
[] [] `GETFAT_C` - Get file architecture and type
[] [] `GETFOV_C` - Get instrument FOV parameters
[] [] `GETFVN_C` - Get instrument FOV parameters, by instrument name
[] [] `GETMSG_C` - Get Error Message
[] [] `GFBAIL_C` - GF, interrupt signal indicator
[] [] `GFCLRH_C` - GF, clear interrupt signal handler status
[] [] `GFDIST_C` - GF, distance search
[] [] `GFEVNT_C` - GF, geometric event finder
[] [] `GFFOVE_C` - GF, is target in FOV?
[] [] `GFILUM_C` - GF, illumination angle search
[] [] `GFINTH_C` - GF, interrupt signal handler
[] [] `GFOCCE_C` - GF, occultation event
[] [] `GFOCLT_C` - GF, find occultation
[] [] `GFPA_C` - GF, phase angle search
[] [] `GFPOSC_C` - GF, observer-target vector coordinate search
[] [] `GFREFN_C` - GF, default refinement estimator
[] [] `GFREPF_C` - GF, progress report finalization
[] [] `GFREPI_C` - GF, progress report initialization
[] [] `GFREPU_C` - GF, progress report update
[] [] `GFRFOV_C` - GF, is ray in FOV?
[] [] `GFRR_C` - GF, range rate search
[] [] `GFSEP_C` - GF, angular separation search
[] [] `GFSNTC_C` - GF, surface intercept vector coordinate search
[] [] `GFSSTP_C` - Geometry finder set step size
[] [] `GFSTEP_C` - Geometry finder step size
[] [] `GFSTOL_C` - GF, set a tolerance value for GF
[] [] `GFSUBC_C` - GF, subpoint vector coordinate search
[] [] `GFTFOV_C` - GF, is target in FOV?
[] [] `GFUDB_C` - GF, user defined boolean
[] [] `GFUDS_C` - GF, user defined scalar
[] [] `GIPOOL_C` - Get integers from the kernel pool
[] [] `GNPOOL_C` - Get names of kernel pool variables

### H
[] [] `HALFPI_C` - Half the value of pi
[] [] `HRMESP_C` - Hermite polynomial interpolation, equal spacing
[] [] `HRMINT_C` - Hermite polynomial interpolation
[] [] `HX2DP_C` - Hexadecimal string to d.p. number

### I
[] [] `IDENT_C` - Return the 3x3 identity matrix
[] [] `ILLUM_C` - Illumination angles
[] [] `ILLUM_PL02` - illumination angles using DSK type 2 plate model
[] [] `ILLUM_PLID``_PL02` - illumination angles using type 2 DSK
[] [] `ILLUMF_C` - Illumination angles, general source, return flags
[] [] `ILLUMG_C` - Illumination angles, general source
[] [] `ILUMIN_C` - Illumination angles
[] [] `INEDPL_C` - Intersection of ellipsoid and plane
[] [] `INELPL_C` - Intersection of ellipse and plane
[] [] `INRYPL_C` - Intersection of ray and plane
[] [] `INSRTC_C` - Insert an item into a character set
[] [] `INSRTD_C` - Insert an item into a double precision set
[] [] `INSRTI_C` - Insert an item into an integer set
[] [] `INTER_C` - Intersection of two sets
[] [] `INTMAX_C` - Largest integer number
[] [] `INTMIN_C` - Smallest integer number
[] [] `INVERT_C` - Invert a 3x3 matrix
[] [] `INVORT_C` - Invert nearly orthogonal matrices
[] [] `INVSTM_C` - Inverse of state transformation matrix
[] [] `ISORDV_C` - Is array an order vector?
[] [] `ISRCHC_C` - Search in a character array
[] [] `ISRCHD_C` - Search in a double precision array
[] [] `ISRCHI_C` - Search in an integer array
[] [] `ISROT_C` - Indicate whether a matrix is a rotation matrix
[] [] `ISWHSP_C` - Determine whether a string is white space

### J
[] [] `J1900_C` - Julian Date of 1900.0 JAN 0.5
[] [] `J1950_C` - Julian Date of 1950.0 JAN 1.0
[] [] `J2000_C` - Julian Date of 2000 JAN 1.5
[] [] `J2100_C` - Julian Date of 2100 JAN 1.5
[] [] `JYEAR_C` - Seconds per julian year


### K
[] [] `KCLEAR_C` - Keeper clear
[] [] `KDATA_C` - Kernel Data
[] [] `KINFO_C` - Kernel Information
[] [] `KPLFRM_C` - Kernel pool frame IDs
[] [] `KTOTAL_C` - Kernel Totals
[] [] `KXTRCT_C` - Extract a substring starting with a keyword


### L
[] [] `LASTNB_C` - Last non-blank character
[] [] `LATCYL_C` - Latitudinal to cylindrical coordinates
[] [] `LATREC_C` - Latitudinal to rectangular coordinates
[] [] `LATSPH_C` - Latitudinal to spherical coordinates
[] [] `LATSRF_C` - Latitudinal grid to surface points
[] [] `LCASE_C` - Convert to lowercase
[] [] `LDPOOL_C` - Load variables from a kernel file into the pool
[] [] `LGRESP_C` - Lagrange interpolation on equally spaced points
[] [] `LGRIND_C` - Lagrange polynomial interpolation with derivative
[] [] `LGRINT_C` - Lagrange polynomial interpolation
[] [] `LIMB_PL02` - Limb using DSK type 2 plate model
[] [] `LIMBPT_C` - Limb points on an extended object
[] [] `LLGRID_PL02` - Lon/lat grid using DSK type 2 plate model
[] [] `LMPOOL_C` - Load variables from memory into the pool
[] [] `LPARSE_C` - Parse items from a list
[] [] `LPARSM_C` - Parse a list of items having multiple delimiters
[] [] `LPARSS_C` - Parse a list of items; return a set
[] [] `LSPCN_C` - Longitude of the sun, planetocentric
[] [] `LSTLEC_C` - Last character element less than or equal to.
[] [] `LSTLED_C` - Last double precision element less than or equal
[] [] `LSTLEI_C` - Last integer element less than or equal to
[] [] `LSTLTC_C` - Last character element less than
[] [] `LSTLTD_C` - Last double precision element less than
[] [] `LSTLTI_C` - Last integer element less than
[] [] `LTIME_C` - Light Time
[] [] `LX4DEC_C` - Scan for decimal number
[] [] `LX4NUM_C` - Scan for number
[] [] `LX4SGN_C` - Scan for signed integer
[] [] `LX4UNS_C` - Scan for unsigned integer
[] [] `LXQSTR_C` - Lex quoted string

### M
[] [] `M2EUL_C` - Matrix to Euler angles
[] [] `M2Q_C` - Matrix to quaternion
[] [] `MATCHI_C` - Match string against wildcard template
[] [] `MATCHW_C` - Match string against wildcard template
[] [] `MAXD_C` - Maximum of a set of double precision values
[] [] `MAXI_C` - Maximum of a set of integers
[] [] `MEQU_C` - Matrix equal to another, 3x3
[] [] `MEQUG_C` - Matrix equal to another, general dimension
[] [] `MIND_C` - Minimum of a set of double precision values
[] [] `MINI_C` - minimum of a set of integers
[] [] `MOVED_C` - Move a double precision array to another
[] [] `MTXM_C` - Matrix transpose times matrix, 3x3
[] [] `MTXMG_C` - Matrix transpose times matrix, general dimension
[] [] `MTXV_C` - Matrix transpose times vector, 3x3
[] [] `MTXVG_C` - Matrix transpose times vector, general dimension
[] [] `MXM_C` - Matrix times matrix, 3x3
[] [] `MXMG_C` - Matrix times matrix, general dimension
[] [] `MXMT_C` - Matrix times matrix transpose, 3x3
[] [] `MXMTG_C` - Matrix times matrix transpose, general dimension
[] [] `MXV_C` - Matrix times vector, 3x3
[] [] `MXVG_C` - Matrix times vector, general dimension

### N
[] [] `NAMFRM_C` - Name to frame
[] [] `NCPOS_C` - NOT Character position
[] [] `NCPOSR_C` - Character position, reverse
[] [] `NEARPT_C` - Nearest point on an ellipsoid
[] [] `NEXTWD_C` - Next word in a character string
[] [] `NPEDLN_C` - Nearest point on ellipsoid to line
[] [] `NPELPT_C` - Nearest point on ellipse to point
[] [] `NPLNPT_C` - Nearest point on line to point
[] [] `NTHWD_C` - n'th word in a character string
[] [] `NVC2PL_C` - Normal vector and constant to plane
[] [] `NVP2PL_C` - Normal vector and point to plane

### O
[] [] `OCCULT_C` - find occultation type at time
[] [] `ORDC_C` - The ordinal position of an element in a set
[] [] `ORDD_C` - The ordinal position of an element in a set
[] [] `ORDERC_C` - Order of a character array
[] [] `ORDERD_C` - Order of a double precision array
[] [] `ORDERI_C` - Order of an integer array
[] [] `ORDI_C` - The ordinal position of an element in a set
[] [] `OSCELT_C` - Determine conic elements from state
[] [] `OSCLTX_C` - Extended osculating elements from state

### P
[] [] `PCKCLS_C` - PCK, close file
[] [] `PCKCOV_C` - PCK coverage
[] [] `PCKFRM_C` - PCK, get reference frame class ID set
[] [] `PCKLOF_C` - PCK, load binary file
[] [] `PCKOPN_C` - PCK, open new file
[] [] `PCKUOF_C` - PCK, unload binary file
[] [] `PCKW02_C` - PCK, write type 2 segment
[] [] `PCPOOL_C` - Put character strings into the kernel pool
[] [] `PDPOOL_C` - Put d.p.'s into the kernel pool
[] [] `PGRREC_C` - Planetographic to rectangular
[] [] `PHASEQ_C` - Phase angle quantity between bodies centers
[] [] `PI_C` - Value of pi
[] [] `PIPOOL_C` - Put integers into the kernel pool
[] [] `PJELPL_C` - Project ellipse onto plane
[] [] `PL2NVC_C` - Plane to normal vector and constant
[] [] `PL2NVP_C` - Plane to normal vector and point
[] [] `PL2PSV_C` - Plane to point and spanning vectors
[] [] `PLTAR_C` - Compute area of plate set
[] [] `PLTEXP_C` - Plate expander
[] [] `PLTNP_C` - Nearest point on triangular plate
[] [] `PLTNRM_C` - DSK, compute outward normal of plate
[] [] `PLTVOL_C` - Compute volume of plate model
[] [] `POLYDS_C` - Compute a Polynomial and its Derivatives
[] [] `POS_C` - Position of substring
[] [] `POSR_C` - Position of substring, reverse search
[] [] `PROMPT_C` - Prompt a user for a string
[] [] `PROP2B_C` - Propagate a two-body solution
[] [] `PRSDP_C` - Parse d.p. number with error checking
[] [] `PRSINT_C` - Parse integer with error checking
[] [] `PSV2PL_C` - Point and spanning vectors to plane
[] [] `PUTCML_C` - Get the command line
[] [] `PXFORM_C` - Position Transformation Matrix
[] [] `PXFRM2_C` - Position Transform Matrix, Different Epochs

### Q
[] [] `Q2M_C` - Quaternion to matrix
[] [] `QCKTRC_C` - Get Quick Traceback
[] [] `QDERIV_C` - Quadratic derivative
[] [] `QDQ2AV_C` - Quaternion and quaternion derivative to a.v.
[] [] `QXQ_C` - Quaternion times quaternion

### R
[] [] `RADREC_C` - Range, RA and DEC to rectangular coordinates
[] [] `RAV2XF_C` - Rotation and angular velocity to transform
[] [] `RAXISA_C` - Rotation axis of a matrix
[] [] `RDTEXT_C` - Read a line from a text file
[] [] `RECAZL_C` - Rectangular coordinates to AZ/EL
[] [] `RECCYL_C` - Rectangular to cylindrical coordinates
[] [] `RECGEO_C` - Rectangular to geodetic
[] [] `RECLAT_C` - Rectangular to latitudinal coordinates
[] [] `RECPGR_C` - Rectangular to planetographic
[] [] `RECRAD_C` - Rectangular coordinates to RA and DEC
[] [] `RECSPH_C` - Rectangular to spherical coordinates
[] [] `REMOVC_C` - Remove an item from a character set
[] [] `REMOVD_C` - Remove an item from a double precision set
[] [] `REMOVI_C` - Remove an item from an integer set
[] [] `REORDC_C` - Reorder a character array
[] [] `REORDD_C` - Reorder a double precision array
[] [] `REORDI_C` - Reorder an integer array
[] [] `REORDL_C` - Reorder a logical array
[] [] `REPMC_C` - Replace marker with character string
[] [] `REPMCT_C` - Replace marker with cardinal text
[] [] `REPMD_C` - Replace marker with double precision number
[] [] `REPMF_C` - Replace marker with formatted d.p. value
[] [] `REPMI_C` - Replace marker with integer
[] [] `REPML_C` - Replace marker with logical value text
[] [] `REPMOT_C` - Replace marker with ordinal text
[] [] `RESET_C` - Reset Error Status
[] [] `RETURN_C` - Immediate Return Indicator
[] [] `ROTATE_C` - Generate a rotation matrix
[] [] `ROTMAT_C` - Rotate a matrix
[] [] `ROTVEC_C` - Transform a vector via a rotation
[] [] `RPD_C` - Radians per degree
[] [] `RQUAD_C` - Roots of a quadratic equation

### S
[] [] `SAELGV_C` - Semi-axes of ellipse from generating vectors
[] [] `SCARD_C` - Set the cardinality of a cell
[] [] `SCDECD_C` - Decode spacecraft clock
[] [] `SCE2C_C` - ET to continuous SCLK ticks
[] [] `SCE2S_C` - ET to SCLK string
[] [] `SCE2T_C` - ET to SCLK ticks
[] [] `SCENCD_C` - Encode spacecraft clock
[] [] `SCFMT_C` - Convert SCLK "ticks" to character clock format
[] [] `SCPART_C` - Spacecraft Clock Partition Information
[] [] `SCS2E_C` - SCLK string to ET
[] [] `SCT2E_C` - SCLK ticks to ET
[] [] `SCTIKS_C` - Convert spacecraft clock string to ticks.
[] [] `SDIFF_C` - Symmetric difference of two sets
[] [] `SET_C` - Compare sets
[] [] `SETMSG_C` - Set Long Error Message
[] [] `SHELLC_C` - Shell sort a character array
[] [] `SHELLD_C` - Shell sort a double precision array
[] [] `SHELLI_C` - Shell sort an integer array
[] [] `SIGERR_C` - Signal Error Condition
[] [] `SINCPT_C` - Surface intercept
[] [] `SIZE_C` - Size of a cell
[] [] `SPD_C` - Seconds per day
[] [] `SPHCYL_C` - Spherical to cylindrical coordinates
[] [] `SPHLAT_C` - Spherical to latitudinal coordinates
[] [] `SPHREC_C` - Spherical to rectangular coordinates
[] [] `SPK14A_C` - SPK, add data to a type 14 segment
[] [] `SPK14B_C` - SPK, begin a type 14 segment
[] [] `SPK14E_C` - SPK, end a type 14 segment
[] [] `SPKACS_C` - SPK, aberration corrected state
[] [] `SPKAPO_C` - S/P Kernel, apparent position only
[] [] `SPKAPP_C` - S/P Kernel, apparent state
[] [] `SPKAPS_C` - SPK, apparent state
[] [] `SPKCLS_C` - SPK, Close file
[] [] `SPKCOV_C` - SPK coverage
[] [] `SPKCPO_C` - SPK, constant position observer state
[] [] `SPKCPT_C` - SPK, constant position target state
[] [] `SPKCVO_C` - SPK, constant velocity observer state
[] [] `SPKCVT_C` - SPK, constant velocity target state
[] [] `SPKEZ_C` - S/P Kernel, easy reader
[] [] `SPKEZP_C` - S/P Kernel, easy position
[] [] `SPKEZR_C` - S/P Kernel, easier reader
[] [] `SPKGEO_C` - S/P Kernel, geometric state
[] [] `SPKGPS_C` - S/P Kernel, geometric position
[] [] `SPKLEF_C` - S/P Kernel, Load ephemeris file
[] [] `SPKLTC_C` - S/P Kernel, light time corrected state
[] [] `SPKOBJ_C` - SPK objects
[] [] `SPKOPA_C` - SPK open for addition
[] [] `SPKOPN_C` - SPK, open new file.
[] [] `SPKPDS_C` - SPK pack descriptor
[] [] `SPKPOS_C` - S/P Kernel, position
[] [] `SPKPVN_C` - S/P Kernel, position and velocity in native frame
[] [] `SPKSFS_C` - S/P Kernel, Select file and segment
[] [] `SPKSSB_C` - S/P Kernel, solar system barycenter
[] [] `SPKSUB_C` - S/P Kernel, subset
[] [] `SPKUDS_C` - SPK - unpack segment descriptor
[] [] `SPKUEF_C` - SPK Kernel, Unload ephemeris file
[] [] `SPKW02_C` - Write SPK segment, type 2
[] [] `SPKW03_C` - Write SPK segment, type 3
[] [] `SPKW05_C` - Write SPK segment, type 5
[] [] `SPKW08_C` - Write SPK segment, type 8
[] [] `SPKW09_C` - Write SPK segment, type 9
[] [] `SPKW10_C` - SPK - write a type 10 segment
[] [] `SPKW12_C` - Write SPK segment, type 12
[] [] `SPKW13_C` - Write SPK segment, type 13
[] [] `SPKW15_C` - SPK, write a type 15 segment
[] [] `SPKW17_C` - SPK, write a type 17 segment
[] [] `SPKW18_C` - Write SPK segment, type 18
[] [] `SPKW20_C` - Write SPK segment, type 20
[] [] `SRFC2S_C` - Surface and body ID codes to surface string
[] [] `SRFCSS_C` - Surface ID and body string to surface string
[] [] `SRFNRM_C` - Map surface points to outward normal vectors
[] [] `SRFREC_C` - Surface to rectangular coordinates
[] [] `SRFS2C_C` - Surface and body strings to surface ID code
[] [] `SRFSCC_C` - Surface string and body ID code to surface ID code
[] [] `SRFXPT_C` - Surface intercept point
[] [] `SSIZE_C` - Set the size of a cell
[] [] `STELAB_C` - Stellar Aberration
[] [] `STLABX_C` - Stellar aberration, transmission case
[] [] `STPOOL_C` - String from pool
[] [] `STR2ET_C` - String to ET
[] [] `SUBPNT_C` - Sub-observer point
[] [] `SUBPT_C` - Sub-observer point
[] [] `SUBPT_PL02` - Sub-observer point using DSK type 2 plate model
[] [] `SUBSLR_C` - Sub-solar point
[] [] `SUBSOL_C` - Sub-solar point
[] [] `SUBSOL_PL02` - Sub-solar point using DSK type 2 plate model
[] [] `SUMAD_C` - Sum of a double precision array
[] [] `SUMAI_C` - Sum of an integer array
[] [] `SURFNM_C` - Surface normal vector on an ellipsoid
[] [] `SURFPT_C` - Surface point on an ellipsoid
[] [] `SURFPV_C` - Surface point and velocity
[] [] `SWPOOL_C` - Set watch on a pool variable
[] [] `SXFORM_C` - State Transformation Matrix
[] [] `SZPOOL_C` - Get size limitations of the kernel pool

### T
[] [] `TANGPT_C` - Ray-ellipsoid tangent point
[] [] `TERM_PL02` - Terminator using DSK type 2 plate model
[] [] `TERMPT_C` - Terminator points on an extended object
[] [] `TIMDEF_C` - Time Software Defaults
[] [] `TIMOUT_C` - Time Output
[] [] `TIPBOD_C` - Transformation, inertial position to bodyfixed
[] [] `TISBOD_C` - Transformation, inertial state to bodyfixed
[] [] `TKFRAM_C` - TK frame, find position rotation
[] [] `TKVRSN_C` - Toolkit version strings
[] [] `TPARCH_C` - Parse check---check format of strings
[] [] `TPARSE_C` - Parse a UTC time string
[] [] `TPICTR_C` - Create a Time Format Picture
[] [] `TRACE_C` - Trace of a 3x3 matrix
[] [] `TRCDEP_C` - Traceback depth
[] [] `TRCNAM_C` - Get module name from traceback
[] [] `TRCOFF_C` - Turn tracing off
[] [] `TRGSEP_C` - Separation quantity from observer
[] [] `TSETYR_C` - Time --- set year expansion boundaries
[] [] `TWOPI_C` - Twice the value of pi
[] [] `TWOVEC_C` - Two vectors defining an orthonormal frame
[] [] `TWOVXF_C` - Two states defining a frame transformation
[] [] `TYEAR_C` - Seconds per tropical year

### U
[] [] `UCASE_C` - Convert to uppercase
[] [] `UCRSS_C` - Unitized cross product, 3x3
[] [] `UDDC_C` - Derivative of function less than zero, df(x)/dx < 0
[] [] `UDDF_C` - First derivative of a function, df(x)/dx
[] [] `UDF_C` - GF, dummy function
[] [] `UNION_C` - Union of two sets
[] [] `UNITIM_C` - Uniform time scale transformation
[] [] `UNLOAD_C` - Unload a kernel
[] [] `UNORM_C` - Unit vector and norm, 3 dimensional
[] [] `UNORMG_C` - Unit vector and norm, general dimension
[] [] `UTC2ET_C` - UTC to Ephemeris Time

### V
[] [] `VADD_C` - Vector addition, 3 dimensional
[] [] `VADDG_C` - Vector addition, general dimension
[] [] `VALID_C` - Validate a set
[] [] `VCRSS_C` - Vector cross product, 3 dimensions
[] [] `VDIST_C` - Vector distance
[] [] `VDISTG_C` - Vector distance, general dimension
[] [] `VDOT_C` - Vector dot product, 3 dimensions
[] [] `VDOTG_C` - Vector dot product, general dimension
[] [] `VEQU_C` - Vector equality, 3 dimensions
[] [] `VEQUG_C` - Vector equality, general dimension
[] [] `VHAT_C` - "V-Hat", unit vector along V, 3 dimensions
[] [] `VHATG_C` - "V-Hat", unit vector along V, general dimension
[] [] `VLCOM3_C` - Vector linear combination, 3 dimensions
[] [] `VLCOM_C` - Vector linear combination, 3 dimensions
[] [] `VLCOMG_C` - Vector linear combination, general dimension
[] [] `VMINUG_C` - Negate vector, "-v", general dimension
[] [] `VMINUS_C` - Negate vector, "-v", 3 dimensions
[] [] `VNORM_C` - Vector norm, 3 dimensions
[] [] `VNORMG_C` - Vector norm, general dimension
[] [] `VPACK_C` - Pack three scalar components into a vector
[] [] `VPERP_C` - Perpendicular component of a 3-vector
[] [] `VPRJP_C` - Vector projection onto plane
[] [] `VPRJPI_C` - Vector projection onto plane, inverted
[] [] `VPROJ_C` - Vector projection, 3 dimensions
[] [] `VPROJG_C` - Vector projection, general dimension
[] [] `VREL_C` - Vector relative difference, 3 dimensions
[] [] `VRELG_C` - Vector relative difference, general dimension
[] [] `VROTV_C` - Vector rotation about an axis
[] [] `VSCL_C` - Vector scaling, 3 dimensions
[] [] `VSCLG_C` - Vector scaling, general dimension
[] [] `VSEP_C` - Angular separation of vectors, 3 dimensions
[] [] `VSEPG_C` - Angular separation of vectors, general dimension
[] [] `VSUB_C` - Vector subtraction, 3 dimensions
[] [] `VSUBG_C` - Vector subtraction, general dimension
[] [] `VTMV_C` - Vector transpose times matrix times vector, 3 dim
[] [] `VTMVG_C` - Vector transpose times matrix times vector
[] [] `VUPACK_C` - Unpack three scalar components from a vector
[] [] `VZERO_C` - Is a vector the zero vector?
[] [] `VZEROG_C` - Is a vector the zero vector? -- general dim.

### W
[] [] `WNCARD_C` - Cardinality of a double precision window
[] [] `WNCOMD_C` - Complement a DP window
[] [] `WNCOND_C` - Contract the intervals of a DP window
[] [] `WNDIFD_C` - Difference two DP windows
[] [] `WNELMD_C` - Element of a DP window
[] [] `WNEXPD_C` - Expand the intervals of a DP window
[] [] `WNEXTD_C` - Extract the endpoints from a DP window
[] [] `WNFETD_C` - Fetch an interval from a DP window
[] [] `WNFILD_C` - Fill small gaps in a DP window
[] [] `WNFLTD_C` - Filter small intervals from a DP window
[] [] `WNINCD_C` - Included in a double precision window
[] [] `WNINSD_C` - Insert an interval into a DP window
[] [] `WNINTD_C` - Intersect two DP windows
[] [] `WNRELD_C` - Compare two DP windows
[] [] `WNSUMD_C` - Summary of a double precision window
[] [] `WNUNID_C` - Union two DP windows
[] [] `WNVALD_C` - Validate a DP window

### X
[] [] `XF2EUL_C` - State transformation to Euler angles
[] [] `XF2RAV_C` - Transform to rotation and angular velocity
[] [] `XFMSTA_C` - Transform state between coordinate systems
[] [] `XPOSE6_C` - Transpose a matrix, 6x6
[] [] `XPOSE_C` - Transpose a matrix, 3x3
[] [] `XPOSEG_C` - Transpose a matrix, general

